mod mml_parser;
mod wav_writer;

fn main() {
    // カエルの歌の楽譜
    let mml = format!("{}{}",
                      "o5 l4 cdef edl2c l4 efga gfl2e",
                      "l4 crcr crcr l8 ccdd eeff l4 ed l2c");
    // 楽譜を一音ずつのVec<Note>に変換
    let notes = mml_parser::parse(mml);
    // WAVファイルへ書き込む
    let bpm = 120.0;
    wav_writer::write("kaeru.wav", notes, bpm);

    // きらきら星の楽譜
    let mml = format!("{}{}{}",
                      "o5 l4 ccgg aal2g l4 ffee ddl2c",
                      "l4 ggff eel2d l4 ggff eel2d",
                      "l4 ccgg aal2g l4 ffee ddl2c");
    let notes = mml_parser::parse(mml);
    let bpm = 120.0;
    wav_writer::write("kirakira.wav", notes, bpm);

    // FFプレリュード
    let mml = format!("{}{}{}{}{}{}{}{}{}{}{}{}",
                      "o4 l8 cdeg o5 cdeg o6 cdeg o7 cdeg",
                      "o8 c o7 gedc o6 gedc o5 gedc o4 ged",
                      "o3 ab o4 ceab o5 ceab o6 ceab o7 ce",
                      "aec o6 baec o5 baec o4 baec o3 b",
                      "a o4 cfga o5 cfga o6 cfga o7 cfg",
                      "agfc o6 agfc o5 agfc o4 agfc",
                      "o3 b o4 dgab o5 dgab o6 dgab o7 dga",
                      "bagd o6 bagd o5 bagd o4 bagd",
                      "o3 A o4 cEgA o5 cEgA o6 cEgA o7 cEg",
                      "AgEc o6 AgEc o5 AgEc o4 AgEc",
                      "o3 B o4 dfaB o5 dfaB o6 dfaB o7 dfa",
                      "Bafd o6 Bafd o5 Bafd o4Bafd");
    let notes = mml_parser::parse(mml);
    let bpm = 180.0;
    wav_writer::write("ff_prelude.wav", notes, bpm);
}

