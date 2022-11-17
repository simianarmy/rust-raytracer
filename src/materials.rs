use crate::color::*;
use crate::lights::*;
use crate::math::F3D;
use crate::tuple::*;
use glm::*;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: F3D,
    pub diffuse: F3D,
    pub specular: F3D,
    pub shininess: F3D,
}

pub fn material(ambient: F3D, diffuse: F3D, specular: F3D, shininess: F3D) -> Material {
    Material {
        color: Color::white(),
        ambient,
        diffuse,
        specular,
        shininess,
    }
}

impl Material {
    pub fn new() -> Material {
        material(0.1, 0.9, 0.9, 200.0)
    }
}

// Phong lighting
pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: Point,
    eyev: Vector,
    normalv: Vector,
) -> Color {
    // combine surface color with lights color/intensity
    let effective_color: Color = material.color * light.intensity;

    // find direction to light source
    let lightv: Vector = normalize(&(light.position - point));

    // compute ambient contribution
    let ambient: Color = effective_color * material.ambient;

    // light_dot_normal represents the cosine of the angle between the light vector and the normal vector. A negative number means the​​   ​# light is on the other side of the surface.
    let light_dot_normal: F3D = lightv.dot(&normalv);
    if light_dot_normal >= 0.0 {
        // compute the diffuse contribution
        let diffuse: Color = effective_color * material.diffuse * light_dot_normal;
        // reflect_dot_eye represents the cosine of the angle between the​​     ​# reflection vector and the eye vector. A negative number means the​​     ​# light reflects away from the eye.
        let reflectv: Vector = crate::tuple::reflect(-lightv, normalv);
        let reflect_dot_eye: F3D = reflectv.dot(&eyev);
        let mut specular = Color::black();

        if reflect_dot_eye >= 0.0 {
            // compute the specular contribution
            let factor: F3D = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
        ambient + diffuse + specular
    } else {
        // no light contribution, diffuse and specular are zero
        ambient
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_eps;

    fn setup() -> (Material, Point) {
        (Material::new(), point_zero())
    }

    #[test]
    fn default_material() {
        let m = Material::new();
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::white());
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let (m, position) = setup();
        let eyev = vector(0.0, 2_f32.sqrt() / 2.0, -2_f32.sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::white());
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), Color::white());
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq_eps!(result.tuple(), Color::new(0.7364, 0.7364, 0.7364).tuple());
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vec() {
        let (m, position) = setup();
        let eyev = vector(0.0, -2_f32.sqrt() / 2.0, -2_f32.sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), Color::white());
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq_eps!(result.tuple(), Color::new(1.6364, 1.6364, 1.6364).tuple());
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), Color::white());
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq_eps!(result.tuple(), Color::new(0.1, 0.1, 0.1).tuple());
    }
}
