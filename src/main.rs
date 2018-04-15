extern crate chrono;
use chrono::prelude::*;

fn format_time(time : DateTime<Local>) -> String {
    time.format("%I:%M %p").to_string()
}

fn sleepcycle_inc(time : DateTime<Local>, num_cycles : i32) -> DateTime<Local>
{
    let going_to_sleep = chrono::Duration::minutes(14);
    let sleep_cycle = chrono::Duration::minutes(90);
    time + sleep_cycle * num_cycles + going_to_sleep
}

fn sleepcycle_dec(time: DateTime<Local>, num_cycles : i32) -> DateTime<Local>
{
    let going_to_sleep = chrono::Duration::minutes(14);
    let sleep_cycle = chrono::Duration::minutes(90);
    time - sleep_cycle * num_cycles - going_to_sleep
}

fn bedtime_if(time : DateTime<Local>) {
    println!("If you want to wake up at {}, you should try to fall asleep at one of the following times:", format_time(time));

    let start = 1;
    let non_incl_end = 7;
    for num_sleepcycles in (start..non_incl_end).rev() {
        if num_sleepcycles != start 
        {
            print!("{} or ", format_time(sleepcycle_dec(time, num_sleepcycles)));
        }
        else 
        {
            print!("{}", format_time(sleepcycle_dec(time, num_sleepcycles)));
        }
    }
    println!()
}
fn bedtime_rn(time : DateTime<Local>) {
    println!("If you head to bed right now, you should try to wake up at one of the following times:");

    let start = 1;
    let end = 6;
    let non_incl_end = 7;
    for num_sleepcycles in start..non_incl_end {
        if num_sleepcycles != end
        {
            print!("{} or ", format_time(sleepcycle_inc(time, num_sleepcycles)));
        }
        else 
        {
            print!("{}", format_time(sleepcycle_inc(time, num_sleepcycles)));
        }
    }
    println!()

}

fn main() {
   let local_now = Local::now(); 
   let wakeup_time = Local::now() + chrono::Duration::hours(9);
   bedtime_rn(local_now);
   println!();
   bedtime_if(wakeup_time);
   println!();
}
