struct Torus {
    torus_points: Vec<(f64,f64,f64)>,   //(x,y,z)
    projection:Array2<f64>,
    luminance:Array2<f64>,
}
use core::{f64::consts::TAU, fmt};
use ndarray::Array2;

const SCREEN_SIZE_X:usize=80;
const SCREEN_SIZE_Y:usize=40;

const DONUT_VIEWER_DISTANCE:f64 = 20.0;

const CENTER_X:usize = SCREEN_SIZE_X as usize/2;
const CENTER_Y:usize = SCREEN_SIZE_Y as usize/2;

#[allow(dead_code)]
impl Torus {


    pub fn create_torus(torus_radius:f64, ring_radius:f64,) -> Torus {
        let mut torus_points_ = Vec::new();
        let projection_= Array2::<f64>::zeros((SCREEN_SIZE_Y,SCREEN_SIZE_X));
        let luminance_= Array2::<f64>::zeros((SCREEN_SIZE_Y,SCREEN_SIZE_X));
        

        const THETA_SPACING:f64 = 0.02;
        const Y_AXIS_SPACING:f64 = 0.07;
           
        let mut y_axis:f64=0.0; //Y_axis= rotation by y axis
        
        while  y_axis < TAU {       // TAU = 2*PI 
           let mut theta:f64 = 0.0; //theta = angle of circle; 
           
            while theta < TAU {
                let circle_params= torus_radius + ring_radius*theta.cos();
                let point:(f64,f64,f64) = ( circle_params*y_axis.cos(), ring_radius*theta.sin(), -circle_params*y_axis.sin() ); 

                torus_points_.push(point);
                theta += THETA_SPACING;
            }
            
            y_axis += Y_AXIS_SPACING;
        }
        return Torus {
            torus_points:torus_points_,
            projection:projection_,
            luminance:luminance_
        };
    }

    
    pub fn print_torus(self){
        for point in self.torus_points{
            println!(" {0:.2} , {1:.2} , {2:.2}  ",point.0,point.1,point.2);
        }
    }

    pub fn rotate(&mut self,a:f64,b:f64){

        let cos_a=a.cos();
        let cos_b=b.cos();
        let sin_a=a.sin();
        let sin_b=b.sin();

        for point in self.torus_points.iter_mut().enumerate(){
            let (_,rotate) = point;
            let (x,y,z)=rotate; 
            
            let result_x= *x*cos_b - sin_b*(*y*cos_a-*z*sin_a);

            let result_y  = *x*sin_b + cos_b*(*y*cos_a-*z*sin_a);

            let result_z = *y*sin_a + *z*cos_a;

            *x=result_x;
            *y=result_y;
            *z=result_z;
        }
    }

    pub fn clear_projection(&mut self){
        for y in 0..SCREEN_SIZE_Y {
            for x in 0..SCREEN_SIZE_X{
                self.projection[[y,x]] = 0.000;
            }
        }
    }

    pub fn set_luminance(&mut self,luminance_point:(f64,f64,f64)){
        let (x,y,z) = luminance_point;

        for point in self.torus_points.iter(){
            
        }
    }

    pub fn calculate_projection(&mut self) ->f64{
        
        let vec = &self.torus_points;

        static mut max_lum_val:f64 = 0.0;

        for point in vec.iter().enumerate(){
            let (_a,(x,y,z)) = point;
            let x_prim = ((SCREEN_SIZE_X as f64*x)/(z+DONUT_VIEWER_DISTANCE)) + CENTER_X as f64;
            let y_prim = ((SCREEN_SIZE_Y as f64*y)/(z+DONUT_VIEWER_DISTANCE)) + CENTER_Y as f64;

            let x_prim = x_prim as usize;
            let y_prim = y_prim as usize;
            if x_prim < SCREEN_SIZE_X && y_prim < SCREEN_SIZE_Y {
                if self.projection[[ y_prim, x_prim ]] < (1.0/z).abs(){
                    self.projection[[ y_prim, x_prim ]] = (1.0/z).abs();
                    
                    unsafe{
                        let luminance_value = -100.0*y+4.0*z;
                        self.luminance[[y_prim,x_prim]] = luminance_value;
                        if max_lum_val < luminance_value  {
                            max_lum_val = luminance_value
                        }
                    }
                }
            }
        }
        unsafe{
            return max_lum_val;
        }

    }

    pub fn display(&mut self){  
        let max_lum_val = self.calculate_projection();

        self.show_torus_frame(max_lum_val);

        self.clear_projection();
    }

    pub fn show_torus_frame (&mut self,max_lum_val:f64){
        let cursor = TerminalCursor::new();
        cursor.blink(false).unwrap();
        cursor.hide().unwrap();
        cursor.goto(0, 0).unwrap();
        for y in 0..SCREEN_SIZE_Y {
            for x in 0..SCREEN_SIZE_X{
                if self.projection[[y,x]] != 0.000 {
                    let l = self.luminance[[y,x]]/max_lum_val;
                    
                    if l>0.95{
                        print!("@");
                        continue;
                    }
                    if l>0.9{
                        print!("$");
                        continue;
                    }
                    if l>0.8{
                        print!("#");
                        continue;
                    }
                    if l>0.7{
                        print!("*");
                        continue;
                    }
                    if l>0.6{
                        print!("!");
                        continue;
                    }
                    if l>0.5{
                        print!("=");
                        continue;
                    }
                    if l>0.4{
                        print!(";");
                        continue;
                    }
                    if l>0.3{
                        print!("~");
                        continue;
                    }
                    if l>0.2{
                        print!(":");
                        continue;
                    }
                    if l>0.1{
                        print!(",");
                        continue;
                    }
                    print!(".");  
                }
                else{
                    print!(" ");
                }    
            }
            println!();
        }
    }
    
}


impl fmt::Display for Torus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.torus_points;

        write!(f, "[\n")?;

        for (count, v) in vec.iter().enumerate() {
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, " ")?; }
            writeln!(f, "({0:.2} , {1:.2} , {2:.2})", v.0, v.1, v.2 )?;
        }
        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]\n")
    }
}

use rand::Rng;
use crossterm_cursor::TerminalCursor;
use std::{thread, time};

fn main() {
    let mut paczek = Torus::create_torus(5.0, 2.0);
    let mut rng = rand::thread_rng();
    let mut angle_1 = rng.gen_range(-0.3..0.3); 
    let mut angle_2 = rng.gen_range(-0.3..0.3); 

    paczek.rotate(1.0,0.0);
    let mut counter = 0;
    loop{
        paczek.rotate(angle_1,angle_2);
        paczek.display();
        thread::sleep(time::Duration::from_millis(60));
        counter+=1;
        if counter == 200 {
            angle_1 = rng.gen_range(-0.3..0.3); 
            angle_2 = rng.gen_range(-0.3..0.3); 
            counter = 0;
        }
    }
}
