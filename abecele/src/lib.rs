extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod numbers {
    use sysfs_gpio::{Direction, Pin};
    use std::thread::sleep;
    use std::time::Duration;
    use std::io;
    
    pub fn zero (){
        let my_led = Pin::new(24);
        loop{
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).expect("wtf");
                sleep(Duration::from_millis(100));
                my_led.set_value(1).unwrap();
                sleep(Duration::from_millis(300));

                my_led.set_value(0).unwrap();
                sleep(Duration::from_millis(100));

                my_led.set_value(1).unwrap();
                sleep(Duration::from_millis(300));
                
                my_led.set_value(0).unwrap();
                sleep(Duration::from_millis(100));

                my_led.set_value(1).unwrap();
                sleep(Duration::from_millis(300));
                
                my_led.set_value(0).unwrap();
                sleep(Duration::from_millis(100));

                my_led.set_value(1).unwrap();
                sleep(Duration::from_millis(300));
                
                my_led.set_value(0).unwrap();
                sleep(Duration::from_millis(100));

                my_led.set_value(1).unwrap();
                sleep(Duration::from_millis(300));
                
                Ok(())});
        }
    }
    pub fn one (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        Ok(())});
    }

    pub fn two (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        Ok(())});
    }
    pub fn three (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        Ok(())});
    }
    pub fn four (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        Ok(())});
    }
    pub fn five (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        Ok(())});
    }
    pub fn six (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        Ok(())});
    }
    pub fn seven (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        Ok(())});
    }
    pub fn eight (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        Ok(())});
    }
    pub fn nine (){
        let my_led = Pin::new(24);
        my_led.with_exported(||{
            my_led.set_direction(Direction::Out).unwrap();
                my_led.set_value(0).unwrap();
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
        
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(400));
                
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(100));

        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        Ok(())});
    }
}