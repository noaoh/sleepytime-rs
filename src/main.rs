extern crate chrono;
use chrono::prelude::*;

fn to_naivetime(datetime : DateTime<Local>) -> NaiveTime {
      NaiveTime::from_hms(datetime.hour(), datetime.minute(), 0) 
}
fn format_time(time : NaiveTime) -> String {
    time.format("%I:%M %p").to_string()
}

fn parse_time(time: &str) -> NaiveTime {
    NaiveTime::parse_from_str(time, "%I:%M %p").unwrap()
}

fn sleepcycle_inc(time : NaiveTime, num_cycles : i32) -> NaiveTime 
{
    let going_to_sleep = chrono::Duration::minutes(15);
    let sleep_cycle = chrono::Duration::minutes(90);
    time + sleep_cycle * num_cycles + going_to_sleep
}

fn sleepcycle_dec(time: NaiveTime, num_cycles : i32) -> NaiveTime
{
    let going_to_sleep = chrono::Duration::minutes(15);
    let sleep_cycle = chrono::Duration::minutes(90);
    time - sleep_cycle * num_cycles - going_to_sleep
}

pub fn bedtime_if(wakeuptime : NaiveTime) {
    println!("If you want to wake up at {}, you should try to fall asleep at one of the following times:", format_time(wakeuptime));

    let start = 1;
    let end = 6;
    for num_sleepcycles in (start..=end).rev() {
        if num_sleepcycles != start {
            print!("{} or ", format_time(sleepcycle_dec(wakeuptime, num_sleepcycles)));
        } else {
            print!("{}", format_time(sleepcycle_dec(wakeuptime, num_sleepcycles)));
        }
    }
    println!()
}

fn wakeuptime_msg(bedtime : NaiveTime) {
    if bedtime.with_second(0) == Some(to_naivetime(Local::now()))
    {
        println!("If you head to bed right now, you should try to wake up at one of the following times:")
    } else {
        println!("If you head to bed at {}, you should try to wake up at one of the following times:", format_time(bedtime));
    }
}

pub fn wakeuptime_if(bedtime : NaiveTime) {
    wakeuptime_msg(bedtime);
    let start = 1;
    let end = 6;
    for num_sleepcycles in start..=end {
        if num_sleepcycles != end
        {
            print!("{} or ", format_time(sleepcycle_inc(bedtime, num_sleepcycles)));
        } else {
            print!("{}", format_time(sleepcycle_inc(bedtime, num_sleepcycles)));
        }
    }
    println!()

}

fn main() {
    let potato : &str = "5:30 AM";
    bedtime_if(parse_time(potato));
    let yeet : &str = "10:30 PM";
    wakeuptime_if(parse_time(yeet));
}
