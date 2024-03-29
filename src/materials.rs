use crate::color::*;
use crate::lights::*;
use crate::math;
use crate::math::F3D;
use crate::object::Object;
use crate::pattern::*;
use crate::tuple;
use crate::tuple::*;
use glm::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: F3D,
    pub diffuse: F3D,
    pub specular: F3D,
    pub shininess: F3D,
    pub reflective: F3D,
    pub transparency: F3D,
    pub refractive_index: F3D,
    pub pattern: Option<TPattern>,
}

impl Material {
    pub fn new(ambient: F3D, diffuse: F3D, specular: F3D, shininess: F3D) -> Material {
        Material {
            color: Color::white(),
            ambient,
            diffuse,
            specular,
            shininess,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }

    pub fn set_pattern(&mut self, pattern: Option<TPattern>) {
        self.pattern = pattern;
    }

    // Phong lighting
    pub fn lighting(
        &self,
        object: &Object,
        light: &Light,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        light_intensity: F3D,
    ) -> Color {
        // use material pattern for color if it exists
        let color = if let Some(p) = &self.pattern {
            p.pattern_at_shape(object, &point)
        } else {
            self.color
        };
        // combine surface color with lights color/intensity
        let effective_color: Color = color * light.intensity();

        // find direction to light source
        let lightv: Vector = normalize(&(light.position() - point));

        // compute ambient contribution
        let ambient: Color = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between the light vector and the normal vector. A negative number means the light is on the other side of the surface.
        let light_dot_normal: F3D = lightv.dot(&normalv);
        if math::f_equals(light_intensity, 0.0) || light_dot_normal < 0.0 {
            // no light contribution, diffuse and specular are zero
            ambient
        } else {
            // compute the diffuse contribution
            let diffuse: Color = effective_color * self.diffuse * light_dot_normal;
            // reflect_dot_eye represents the cosine of the angle between the reflection vector and the eye vector. A negative number means the light reflects away from the eye.
            let reflectv: Vector = tuple::reflect(-lightv, *normalv);
            let reflect_dot_eye: F3D = reflectv.dot(&eyev);
            let mut specular = Color::black();

            if reflect_dot_eye >= 0.0 {
                // compute the specular contribution
                let factor: F3D = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor;
            }
            (ambient + diffuse + specular) * light_intensity
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(0.1, 0.9, 0.9, 200.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_eps;
    use crate::pattern::stripe::stripe_pattern;
    use crate::shapes::sphere::*;

    fn setup() -> (Material, Point, Object) {
        (Material::default(), point_zero(), sphere())
    }

    #[test]
    fn default_material() {
        let (m, _, _) = setup();
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position, object) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::white());
        let result = m.lighting(&object, &light, &position, &eyev, &normalv, 1.0);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let (m, position, object) = setup();
        let eyev = vector(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::white());
        let result = m.lighting(&object, &light, &position, &eyev, &normalv, 1.0);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let (m, position, object) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), Color::white());
        let result = m.lighting(&object, &light, &position, &eyev, &normalv, 1.0);
        assert_eq_eps!(result.tuple(), Color::new(0.7364, 0.7364, 0.7364).tuple());
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vec() {
        let (m, position, object) = setup();
        let eyev = vector(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), Color::white());
        let result = m.lighting(&object, &light, &position, &eyev, &normalv, 1.0);
        assert_eq_eps!(result.tuple(), Color::new(1.6364, 1.6364, 1.6364).tuple());
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position, object) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), Color::white());
        let result = m.lighting(&object, &light, &position, &eyev, &normalv, 1.0);
        assert_eq_eps!(result.tuple(), Color::new(0.1, 0.1, 0.1).tuple());
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let (m, position, object) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::white());
        let result = m.lighting(&object, &light, &position, &eyev, &normalv, 0.0);
        assert_eq_eps!(result.tuple(), Color::new(0.1, 0.1, 0.1).tuple());
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = Material::default();
        // TODO: implement set_pattern()
        m.pattern = Some(TPattern::Stripe(stripe_pattern(
            Color::white(),
            Color::black(),
        )));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let (_, _, object) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::white());
        let c1 = m.lighting(
            &object.clone(),
            &light,
            &point(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            0.0,
        );
        let c2 = m.lighting(&object, &light, &point(1.1, 0.0, 0.0), &eyev, &normalv, 1.0);
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }

    #[test]
    fn default_reflective_value() {
        let m = Material::default();
        assert_eq!(m.reflective, 0.0);
    }
}
