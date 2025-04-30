// [drawing_gif/src/main.rs]

// 1. gif 파일을 만들기 위한 라이브러리 gif를 Cargo.toml에 추가한다

// [dependencies]
// gif = "0.11.0"
// rand = "0.8.5"
// rand = "0.8.5"는 랜덤한 수를 생성하기 위한 라이브러리입니다.
// gif = "0.11.0"는 gif 파일을 만들기 위한 라이브러리입니다.
// gif 라이브러리의 버전은 0.11.0으로 설정했습니다.
// Cargo.toml 파일에 다음과 같이 추가합니다.

// 4. use 키워드로 경로를 범위로 가져온다
// 외부에서 작성된 함수를 호출할 때 경로를 생략하기 위해 use 키워드로 다음과 같이 선언하였습니다.
use gif::{Frame, Encoder, Repeat};
use std::fs::File;
use std::borrow::Cow;
use rand::{thread_rng, Rng};

// 5. 여러 함수에서 사용하는 수치들을 const로 선언했다
// const 키워드로 자주 사용하거나 바꿔보고 싶은 변수 값을 다음과 같이 상수로 선언했습니다.
const FILE_NAME: &str = "suns_motion-009-sun";
const GRAVITATIONAL_CONSTANT: f64 = 6.67e-11;   // 만유인력 상수
const NUM_OF_SUNS: usize = 1000;                // 시뮬레이션 하고자하는 항성의 개수
const INIT_RANGE: f64 = 1.0e12;                 //초기 위치 분포 범위(-, +)
const INIT_RANGE_V: f64 = 0.00001;              //초기 속도 분포 범위(-, +)

const VIEW_RANGE: f64 = 10000.0 * INIT_RANGE;   //그림 풍경 범위(-, +)
const DT: f64 = 3.15e7;                     // 3.15e7 초 = 약 1년
                                            // 너무 작으면 급가속 되기 쉬움
const MASS_OF_OUR_SUN: f64 = 1.99e30;       //우리태양의 질량 
                    // mass: (0.001*MASS_OF_OUR_SUN)..(0.1*MASS_OF_OUR_SUN)


                    
// 6. 항성의 질량, 위치, 속도를 저장할 구조체를 만들었다
// 2차원 공간에서 항성들의 위치를 저장할 구조체와 gif 파일 제작 방법을 전달할 구조체를 다음과 같이 만들었습니다.
#[derive(Clone,Copy,Debug)]
struct GifImage {
    width: u16,
    height: u16,
    color_map: [u8; 6],
    num_of_frames: usize,
    size_of_a_frame: usize,
    scale_x_left: f64,
    scale_x_right: f64,
    scale_y_top: f64,
    scale_y_bottom: f64,
    time_interval: f64
}
impl GifImage {
    fn new() -> GifImage {
        GifImage { 
            width: 300,
            height: 300,
            color_map: [0xFF, 0xFF, 0xFF, 0, 0, 0],
            num_of_frames: 1000,
            size_of_a_frame: 10000,
            scale_x_left: -1.0 * VIEW_RANGE,
            scale_x_right: VIEW_RANGE,
            scale_y_top: VIEW_RANGE,
            scale_y_bottom: -1.0 * VIEW_RANGE,
            time_interval: DT            
        }
    }
}
fn init_gif(set: &mut GifImage) -> bool {
    set.width = 600;  //화면의 가로 크기
    set.height = 600; //화면의 세로 크기
    set.color_map = [0, 0, 0, 0xFF, 0xFF, 0xFF];
    set.num_of_frames = 1000; //gif파일 프레임 수
    set.size_of_a_frame = 
        set.width as usize * set.height as usize;
    set.scale_x_left = -1.0 * VIEW_RANGE;
    set.scale_x_right = VIEW_RANGE;
    set.scale_y_top = VIEW_RANGE;
    set.scale_y_bottom = -1.0 * VIEW_RANGE;
    set.time_interval = DT;
    true
}

#[derive(Clone,Copy,Debug)]
struct SunStruct {
    mass: f64,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ax: f64,
    ay: f64,
}
impl SunStruct {
    fn new() -> SunStruct {
        SunStruct {mass: 0., x: 0., y: 0.,
            vx: 0., vy: 0., ax: 0., ay: 0.}
    }
}

// 7. main() 함수로 시뮬레이션 순서를 정한다
// 프로그램의 큰 흐름을 다음과 같이 main() 함수에 다른 함수를 부르는 형태로 작성했습니다.
fn main() {
    let mut sun_array = [SunStruct::new(); NUM_OF_SUNS];
    let mut gif_settings = GifImage::new();
    
    let gif_name = format!("{}{}", FILE_NAME, ".gif");
        
    println!("Starting...........");
    println!("{}",FILE_NAME);
    init_data(&mut sun_array);
    //init_data_5(&mut sun_array); //for after all
    println!("init_data(): O.K.");
    init_gif(&mut gif_settings);
    println!("init_gif(): O.K.");
    make_gif_sun(&gif_settings, &gif_name, &mut sun_array); 
    println!("make_gif_sun(): O.K.");
    println!("finished ALL !");
}


// 8. 항성들에게 처음 값을 부여하는 함수 init_data()
// 다음과 같이 init_data() 함수를 만들어 각 항성의 질량, 위치(x, y), 속도(vx, vy)를 일정 범위 안에 있는 값으로 랜덤하게 부여했습니다.
fn init_data(sun: &mut [SunStruct]) -> bool {
    let mut rng = thread_rng();
    let _sun_num = sun.len();
    for i in 0.._sun_num {
        sun[i].mass = rng.gen_range((0.001*MASS_OF_OUR_SUN)..(0.1*MASS_OF_OUR_SUN));
        // 초기 입자 위치 범위(m)
        sun[i].x = rng.gen_range((-1.0 * INIT_RANGE)..INIT_RANGE);
        sun[i].y = rng.gen_range((-1.0 * INIT_RANGE)..INIT_RANGE);
        // 초기 입자 속도 범위(m/s)
        sun[i].vx = rng.gen_range((-1.0 * INIT_RANGE_V)..INIT_RANGE_V);
        sun[i].vy = rng.gen_range((-1.0 * INIT_RANGE_V)..INIT_RANGE_V);        
    }
    true
}

// 9. 항성 간의 중력에 의한 가속도 계산
// summation_acceleration() 함수는 각 항성에 대하여 나머지 모든 항성들 사이의 만유인력에 의한 가속도 벡터를 합하는 함수
fn summation_accelerations(sun_array: &mut [SunStruct]) -> bool {
    let mut dx: f64;
    let mut dy: f64;
    let mut r: f64;
    let mut r_pow2: f64;
    let mut a: f64; //acceleration
    let _sun_num = sun_array.len();
    for i in 0.._sun_num{
        if sun_array[i].mass == 0.0 { continue }
        sun_array[i].ax = 0.;
        sun_array[i].ay = 0.;
        // acceleration due to other suns
        for j in 0.._sun_num{
            if  i == j {continue}
            if sun_array[j].mass == 0.0 {continue} 
            dx = sun_array[j].x - sun_array[i].x;
            dy = sun_array[j].y - sun_array[i].y;
            r_pow2 = dx.powi(2) + dy.powi(2);
            r = r_pow2.sqrt();
            a = GRAVITATIONAL_CONSTANT * sun_array[j].mass / r_pow2; 
            sun_array[i].ax = sun_array[i].ax + (a * dx / r) ; //a*cos()
            sun_array[i].ay = sun_array[i].ay + (a * dy / r) ; //a*sin()
        }
    }
    true
}

// 10. 위치와 속도 변화를 계산
// 상대적으로 짧은 시간 동안 항성이 등가속도 운동에 가까운 위치 변화를 한다는 가정하에 새로운 위치 좌표를 다음과같은 방식으로 계산하였습니다.
fn change_positions_and_velocity(sun_array: &mut [SunStruct], set: &GifImage) -> bool {
    let dt_pow2 = set.time_interval.powi(2);
    let sun_num = sun_array.len();
    for i in 0..sun_num { 
        if sun_array[i].mass == 0.0 { continue }
        sun_array[i].x = sun_array[i].x + (sun_array[i].vx * set.time_interval) 
            + (0.5 * sun_array[i].ax * dt_pow2);
        sun_array[i].y = sun_array[i].y + (sun_array[i].vy * set.time_interval) 
            + (0.5 * sun_array[i].ay * dt_pow2);
        
        if sun_array[i].x > set.scale_x_right {sun_array[i].mass = 0.0}
        else if sun_array[i].x < set.scale_x_left {sun_array[i].mass = 0.0}
        if sun_array[i].y > set.scale_y_top {sun_array[i].mass = 0.0}
        else if sun_array[i].y < set.scale_y_bottom {sun_array[i].mass = 0.0}
    }
    change_velocity(sun_array, set);
    true
}
fn change_velocity(sun_array: &mut [SunStruct], set: &GifImage) -> bool {
    let sun_num = sun_array.len();
    for i in 0..sun_num { 
        if sun_array[i].mass == 0.0 { continue }
        sun_array[i].vx = sun_array[i].vx + (sun_array[i].ax * set.time_interval);
        sun_array[i].vy = sun_array[i].vy + (sun_array[i].ay * set.time_interval);
    }
    true
}

// 11. 하나의 결과
// 1000개의 항성에게 적당한 질량을 부과한 후 일정 영역 안에 랜덤하게 배치하도록 코딩한 후 Ctrl-F5를 눌러 컴파일+실행 시키면 gif 파일이 얻어집니다. 




fn make_gif_sun(set: &GifImage, file_name: &str, sun: &mut [SunStruct] ) {
    let num_of_sun = sun.len();
    let mut frames: Vec<Vec<u8>> = Vec::new();
    for _i in 0..set.num_of_frames{
        frames.push(vec![0; set.size_of_a_frame]); 
    }
    let mut frame_x: usize = 0; //LEFT_TOP 좌표가 (0.0)인 좌표변환용
    let mut frame_y: usize = 0;
    let scale_dx: f64 = set.scale_x_right - set.scale_x_left;
    let scale_dy: f64 = set.scale_y_top - set.scale_y_bottom;
    
    for frame in 0..set.num_of_frames {
        for i in 0..num_of_sun{
            if sun[i].mass == 0. { continue}
            let xx = sun[i].x - set.scale_x_left;
            let yy = sun[i].y - set.scale_y_bottom;
            frame_x = (xx * set.width as f64 / scale_dx) as usize;
            frame_y = (yy * set.height as f64 / scale_dy) as usize;
            if frame_x >= set.width as usize {
                frame_x = (set.width - 1) as usize
            }
            if frame_y >= set.height as usize {
                frame_y = (set.height - 1) as usize
            }
            
            let p: usize = set.width as usize * frame_y + frame_x;
            frames[frame][p] = 1;
        }
        println!("caculated frame {} of {} ", frame+1, set.num_of_frames);
        summation_accelerations(sun);
        change_positions_and_velocity(sun, set);
    }
    let mut image = File::create(file_name).unwrap();
    let mut encoder = Encoder::new(&mut image, set.width, set.height, &set.color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    let mut k = 1;
    for state in &frames {
        let mut a_frame = Frame::default();
        a_frame.width = set.width; 
        a_frame.height = set.height;  
        a_frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&a_frame).unwrap();   
        println!("encoded frame {} of {} ", k, set.num_of_frames);
        k = k + 1;
    }
}

