use proconio::input;

pub fn print_music_list() {
    println!("
0: 終了
1: 窓辺のドロップス
2: 星降る
3: 寝落ち
4: 露草
5: 残暑
    ");
}

pub fn music_path_handler(num: u32) -> (&'static str, u64) {
    match num {
        1 => ("assets/drops.mp3", 135),
        2 => ("assets/hoshihuru.mp3", 160),
        3 => ("assets/neochi.mp3", 115),
        4 => ("assets/rokusa.mp3", 155),
        5 => ("assets/zansho.mp3", 170),
        _ => {
            panic!();
        }
    }
}

pub fn select_number() -> u32 {
    println!("What music do you want?");
    input! {
        num: u32,
    }
    
    num
}