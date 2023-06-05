use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.res_type, 0);
        assert_eq!(v.atom_type, 0);
        assert_eq!(v.chain_id, 0);
    }

    #[test]
    fn test_add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
        assert_eq!(result.res_type, 0);
        assert_eq!(result.atom_type, 0);
        assert_eq!(result.chain_id, 0);
    }

    #[test]
    fn test_sub_vec3() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
        assert_eq!(result.res_type, 0);
        assert_eq!(result.atom_type, 0);
        assert_eq!(result.chain_id, 0);
    }

    #[test]
    fn test_mul_vec3() {
        let mut v = Vec3::new(2.0, 3.0, 4.0);
        v.mul(2.0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, 6.0);
        assert_eq!(v.z, 8.0);
        assert_eq!(v.res_type, 0);
        assert_eq!(v.atom_type, 0);
        assert_eq!(v.chain_id, 0);
    }

    #[test]
    fn test_div_vec3() {
        let mut v = Vec3::new(4.0, 6.0, 8.0);
        v.div(2.0);
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 4.0);
        assert_eq!(v.res_type, 0);
        assert_eq!(v.atom_type, 0);
        assert_eq!(v.chain_id, 0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        let result = v.length_squared();
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_normalized() {
        let v = Vec3::new(3.0, 4.0.
