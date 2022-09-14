// use numpy::*;
// use ndarray::*;

// pub mod math{
use std::f64::consts::PI;

// use numpy::*;
use ndarray::*;
use rand::*;


pub fn clip(mut vec: Vec<f64>, high: f64, low: f64) -> Vec<f64> {
    // this can't be right? 
    vec = vec.into_iter().map(|x: f64| if x > high {high} else if x < low {low} else {x}).collect::<Vec<f64>>();
    return vec
}

pub fn trace(arr: Array2<f64>) -> f64 {
    let diag = arr.diag();
    diag.into_iter().sum()
}

/// multiply elementwise vec a * vec b
pub fn element_mult_vec(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    assert!(a.len() == b.len(), "length of a did not match length of b");
    let mut z: Vec<f64> = Vec::new();

    for i in 0..a.len() {
        let x = a[i];
        let y = b[i];

        z.push(x * y);
    }

    return z;
}

    /// multiply elementwise vec a * vec b
    pub fn element_div_vec(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        assert!(a.len() == b.len(), "length of a did not match length of b");
        let mut z: Vec<f64> = Vec::new();

        for i in 0..a.len() {
            let x = a[i];
            let y = b[i];

            z.push(x / y);
        }

        return z;
    }

/// subtract elementwise vec a - vec b
pub fn element_sub_vec(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    assert!(a.len() == b.len(), "length of a did not match length of b");
    let mut z: Vec<f64> = Vec::new();

    for i in 0..a.len() {
        let x = a[i];
        let y = b[i];

        z.push(x - y);
    }

    return z
}

/// add elementwise vec a + vec b
pub fn element_add_vec(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    assert!(a.len() == b.len(), "length of a did not match length of b");
    let mut z: Vec<f64> = Vec::new();

    for i in 0..a.len() {
        let x = a[i];
        let y = b[i];

        z.push(x + y);
    }

    return z;
}

/// subtract elements of two vecs to get dist
pub fn get_dist(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    element_sub_vec(a, b)
}

/// vector projection of two vecs and an optional mag_squared
pub fn vector_projection(vec: Vec<f64>, dest_vec: Vec<f64>, mag_squared: Option<f64>) -> Vec<f64> {
    assert!(vec.len() == dest_vec.len(), "length of a did not match length of b");
    let mut _mag_squared: f64 = 0.;
    if mag_squared.is_some() {
        if mag_squared.unwrap() == 0. {
            return dest_vec;
        }
        else {
            _mag_squared = mag_squared.unwrap();
        }
    }
    else {
        let norm = norm_func(vec.clone());
        if norm == 0. {
            return dest_vec;
        }
        _mag_squared = norm * norm;
    }

    let dot: Vec<f64> = element_mult_vec(vec.clone(), dest_vec.clone());
    let dot_prod: f64 = dot.iter().sum::<f64>();

    let part: f64 = dot_prod/_mag_squared;
    let projection: Vec<f64> = dest_vec.clone()
                                        .into_iter()
                                        .map(|x: f64| x*part)
                                        .collect::<Vec<f64>>();

    return projection;
}

/// get norm of vec
pub fn norm_func(nums: Vec<f64>) -> f64 {
    let norm_val: f64 = nums.clone()
                            .into_iter()
                            .map(|x: f64| x.powf(2.))
                            .collect::<Vec<f64>>()
                            .iter()
                            .sum::<f64>()
                            .sqrt();
    norm_val
}

pub fn scalar_projection(vec: Vec<f64>, dest_vec: Vec<f64>) -> f64 {
    let norm = norm_func(dest_vec.clone());
    if norm == 0. {
        return 0.;
    }
    return (element_mult_vec(vec.clone(), dest_vec.clone()).iter()
                                                                        .sum::<f64>())/norm;
}

pub fn squared_vecmag(vec: Vec<f64>) -> f64 {
    norm_func(vec).powi(2)
}

pub fn vecmag(vec: Vec<f64>) -> f64 {
    norm_func(vec)
}

pub fn unitvec(vec: Vec<f64>) -> Vec<f64> {
    let vecm: f64 = norm_func(vec.clone());
    let mut res: Vec<f64> = Vec::new();
    for i in vec {
        res.push(i/vecm);
    }
    return res;
}

pub fn cosine_simularity(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let a_norm = norm_func(a.clone()).sqrt();
    let b_norm = norm_func(b.clone()).sqrt();
    
    let mut a_vec: Vec<f64> = Vec::new();
    for i in a {
        a_vec.push(i/a_norm);
    }

    let mut b_vec: Vec<f64> = Vec::new();
    for i in b {
        b_vec.push(i/b_norm);
    }

    let mut res: Vec<f64> = Vec::new();
    // this can be used for other stuff too?
    for (a, b) in a_vec.iter_mut().zip(b_vec.iter()) {
        res.push(*a**b);
    }

    return res.iter().sum();
}

pub fn quat_to_euler(quat: Vec<f64>) -> Vec<f64> {
    assert!(quat.len() == 4, "nums is not the correct shape");

    let w: f64 = quat[0];
    let x: f64 = quat[1];
    let y: f64 = quat[2];
    let z: f64 = quat[3];

    let sinr_cosp: f64 = 2. * (w * x + y * z);
    let cosr_cosp: f64 = 1. - 2. * (x * x + y * y);
    let sinp: f64 = 2. * (w * y + x * y);
    let siny_cosp: f64 = 2. * (w * z + x * y);
    let cosy_cosp: f64 = 1. - 2. * (y * y + z * z);
    let roll: f64 = sinr_cosp.atan2(cosr_cosp);

    let pitch: f64;
    if sinp.abs() > 1. {
        pitch = PI / 2.;
    }
    else {
        pitch = sinp.asin();
    }

    let yaw: f64 = siny_cosp.atan2(cosy_cosp);

    vec![-pitch, yaw, -roll]
}

/// quat Vec to rotation matrix Array2
pub fn quat_to_rot_mtx(nums: Vec<f64>) -> Array2<f64> {
    let mut theta = Array2::<f64>::zeros((3, 3));
    
    assert!(nums.len() == 4, "nums is not the correct shape");

    let norm: f64 = nums.clone()
                        .into_iter()
                        .map(|x: f64| x.powf(2.))
                        .collect::<Vec<f64>>()
                        .iter()
                        .sum();
    let s: f64 = 1.0 / norm;

    let w: &f64 = &nums[0];
    let x: &f64 = &nums[1];
    let y: &f64 = &nums[2];
    let z: &f64 = &nums[3];

    // front direction
    theta[[0, 0]] = 1. - 2. * s * (y * y + z * z);
    theta[[1, 0]] = 2. * s * (x * y + z * w);
    theta[[2, 0]] = 2. * s * (x * z - y * w);

    // left direction
    theta[[0, 1]] = 2. * s * (x * y - z * w);
    theta[[1, 1]] = 1. - 2. * s * (x * x + z * z);
    theta[[2, 1]] = 2. * s * (y * z + x * w);

    // up direction
    theta[[0, 2]] = 2. * s * (x * z + y * w);
    theta[[1, 2]] = 2. * s * (y * z - x * w);
    theta[[2, 2]] = 1. - 2. * s * (x * x + y * y);

    // let theta_arr = theta.to_pyarray(py);

    return theta;
}

pub fn rotation_to_quaternion(m: Array2<f64>) -> Array1<f64> {
    let trace: f64 = trace(m.clone());
    let mut q: Array1<f64> = Array1::<f64>::zeros(4);

    if trace > 0. {
        let mut s: f64 = (trace + 1.).powf(0.5);
        q[0] = s * 0.5;
        s = 0.5 / s;
        q[1] = (m[[2, 1]] - m[[1, 2]]) * s;
        q[2] = (m[[0, 2]] - m[[2, 0]]) * s;
        q[3] = (m[[1, 0]] - m[[0, 1]]) * s;
    }
    else {
        if m[[0, 0]] >= m[[1, 1]] && m[[0, 0]] >= m[[2, 2]] {
            let s: f64 = (1. + m[[0, 0]] - m[[1, 1]] - m[[2, 2]]).powf(0.5);
            let inv_s: f64 = 0.5 / s;
            q[1] = 0.5 * s;
            q[2] = (m[[1, 0]] + m[[0, 1]]) * inv_s;
            q[3] = (m[[2, 0]] + m[[0, 2]]) * inv_s;
            q[0] = (m[[2, 1]] - m[[1, 2]]) * inv_s;
        }
        else if m[[1, 1]] > m[[2, 2]] {
            let s: f64 = (1. + m[[1, 1]] - m[[0, 0]] - m[[2, 2]]).powf(0.5);
            let inv_s: f64 = 0.5 / s;
            q[1] = (m[[0, 1]] + m[[1, 0]]) * inv_s;
            q[2] = 0.5 * s;
            q[3] = (m[[1, 2]] + m[[2, 1]]) * inv_s;
            q[0] = (m[[0, 2]] - m[[2, 0]]) * inv_s; 
        }
        else {
            let s: f64 = (1. + m[[2, 2]] - m[[0, 0]] - m[[1, 1]]).powf(0.5);
            let inv_s: f64 = 0.5 / s;
            q[1] = (m[[0, 2]] + m[[2, 0]]) * inv_s;
            q[2] = (m[[1, 2]] + m[[2, 1]]) * inv_s;
            q[3] = 0.5 * s;
            q[0] = (m[[1, 0]] - m[[0, 1]]) * inv_s;
        }
    }
    return -q;
}

pub fn euler_to_rotation(pyr: Array1<f64>) -> Array2<f64> {
    // this probably needs a revisit for ownership purposes
    let mut pyr_cos = pyr.clone();
    let mut pyr_sin = pyr.clone();

    let mut res: Vec<f64> = Vec::new();
    for i in pyr_cos.iter_mut() {
        res.push(i.cos());
    }
    let cp: f64 = res[0];
    let cy: f64 = res[1];
    let cr: f64 = res[2];
    res.clear();

    for i in pyr_sin.iter_mut() {
        res.push(i.sin());
    }
    let sp: f64 = res[0];
    let sy: f64 = res[1];
    let sr: f64 = res[2];

    let mut theta = Array2::<f64>::zeros((3, 3));

    // front
    theta[[0, 0]] = cp * cy;
    theta[[1, 0]] = cp * sy;
    theta[[2, 0]] = sp;

    // left
    theta[[0, 1]] = cy * sp * sr - cr * sy;
    theta[[1, 1]] = sy * sp * sr + cr * cy;
    theta[[2, 1]] = -cp * sr;

    // up
    theta[[0, 2]] = -cr * cy * sp - sr * sy;
    theta[[1, 2]] = -cr * sy * sp + sr * cy;
    theta[[2, 2]] = cp * cr;

    return theta;
}

pub fn rand_uvec3() -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
    let mut rng = thread_rng();
    let rand_num = rng.gen_range((0.)..(1.));
    for _ in 0..3 {
        vec.push(rand_num - 0.5);
    }
    let norm_vec: f64 = norm_func(vec.clone());
    for i in vec.iter_mut() {
        *i = *i/norm_vec;
    }
    return vec;
}

pub fn rand_vec3(max_norm: f64) -> Vec<f64> {
    let mut rng = thread_rng();
    let mut res: Vec<f64> = Vec::new();
    for i in res.iter_mut() {
        let rand_num: f64 = rng.gen_range((0.)..(1.));
        let partial: f64 = rand_num * max_norm;
        *i = *i*partial;
    }
    return res;
}
// }