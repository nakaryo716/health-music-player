mod handlers;

use std::{io::BufReader, fs::File, thread};
use rodio::{OutputStream, Decoder, Source};
use handlers::{print_music_list, music_path_handler, select_number};

fn main() {
    print_music_list();
    
    loop{
        let music_number = select_number();

        if music_number == 0 {
            println!("See you");
            break;
        }

        // 選択された曲のファイルがあるパスと曲の時間を取得
        let (selected_music_path, music_time) = music_path_handler(music_number);

        // 物理サウンドデバイスの出力ストリームハンドルを取得
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let file = BufReader::new(File::open(selected_music_path).unwrap());
        let source = Decoder::new(file).unwrap();

        let music_thread = thread::spawn(move || {
            let _ = stream_handle.play_raw(source.convert_samples());

            println!("now playing...");

            // 曲は別のオーディオスレッドで流れるようになっているため
            // このスレットを曲の時間分だけスリープ
            thread::sleep(std::time::Duration::from_secs(music_time));

            println!("finish");
        });

        music_thread.join().unwrap();
    }
}
