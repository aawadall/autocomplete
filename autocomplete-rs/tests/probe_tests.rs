use std::thread;
use std::time::Duration;
use autocomplete_rs::probe::{Probe, NopProbe, TimerProbe};

#[test]
fn test_nop_probe() {
    let mut probe = NopProbe;
    // These should not panic
    probe.start(0);
    probe.stop(0);
}

#[test]
fn test_timer_probe_single() {
    let mut probe = TimerProbe::new(1);
    
    probe.start(0);
    thread::sleep(Duration::from_millis(100));
    probe.stop(0);
    
    let duration = probe.get_duration(0);
    assert!(duration >= Duration::from_millis(100));
}

#[test]
fn test_timer_probe_multiple() {
    let mut probe = TimerProbe::new(3);
    
    // Timer 0
    probe.start(0);
    thread::sleep(Duration::from_millis(100));
    probe.stop(0);
    
    // Timer 1
    probe.start(1);
    thread::sleep(Duration::from_millis(200));
    probe.stop(1);
    
    // Timer 2
    probe.start(2);
    thread::sleep(Duration::from_millis(300));
    probe.stop(2);
    
    assert!(probe.get_duration(0) >= Duration::from_millis(100));
    assert!(probe.get_duration(1) >= Duration::from_millis(200));
    assert!(probe.get_duration(2) >= Duration::from_millis(300));
}

#[test]
fn test_timer_probe_accumulation() {
    let mut probe = TimerProbe::new(1);
    
    // First interval
    probe.start(0);
    thread::sleep(Duration::from_millis(100));
    probe.stop(0);
    
    // Second interval
    probe.start(0);
    thread::sleep(Duration::from_millis(100));
    probe.stop(0);
    
    let duration = probe.get_duration(0);
    assert!(duration >= Duration::from_millis(200));
}

#[test]
#[should_panic(expected = "Timer ID out of bounds")]
fn test_timer_probe_invalid_id() {
    let mut probe = TimerProbe::new(1);
    probe.start(1); // Should panic as we only have timer 0
}

#[test]
#[should_panic(expected = "Timer ID out of bounds")]
fn test_timer_probe_get_invalid_id() {
    let probe = TimerProbe::new(1);
    probe.get_duration(1); // Should panic as we only have timer 0
} 