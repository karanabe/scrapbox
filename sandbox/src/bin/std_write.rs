// 自前のトレイトを定義する
trait MyWrite {
    fn my_write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn my_flush(&mut self) -> std::io::Result<()>;
}

// std::io::Writeを実装した型に対して自前のトレイトを実装する
impl<W: std::io::Write> MyWrite for W {
    fn my_write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // std::io::Writeのwriteメソッドを呼ぶ
        self.write(buf)
    }

    fn my_flush(&mut self) -> std::io::Result<()> {
        // std::io::Writeのflushメソッドを呼ぶ
        self.flush()
    }
}

// 自前のトレイトを使って標準出力に書き込む関数
fn write_to_stdout<T: MyWrite>(mut writer: T) -> std::io::Result<()> {
    // 自前のトレイトのmy_writeメソッドを呼ぶ
    writer.my_write(b"Hello, world!\n")?;
    // 自前のトレイトのmy_flushメソッドを呼ぶ
    writer.my_flush()
}

// 自前の型を定義する
struct MyWriter {
    buffer: Vec<u8>,
}

// 自前の型にWriteを実装する
impl std::io::Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // バッファにデータを追加する
        self.buffer.extend_from_slice(buf);
        // 書き込んだバイト数を返す
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // バッファを空にする
        self.buffer.clear();
        // Okを返す
        Ok(())
    }
}

// 自前の型を使って標準出力に書き込む関数
fn write_to_stdout2<T>(mut writer: T)
where
    T: std::io::Write,
{
    // Writeのwriteメソッドを呼ぶ
    let _ = writer.write(b"Hello, world!\n");
    // Writeのflushメソッドを呼ぶ
    // writer.flush()
}

fn main() -> std::io::Result<()> {
    // 標準出力を取得する
    let stdout = std::io::stdout();
    // 標準出力に書き込む関数を呼ぶ
    let _ = write_to_stdout(stdout);

    let mut my_writer = MyWriter { buffer: Vec::new() };
    // 自前の型を標準出力に書き込む関数に渡す
    write_to_stdout2(&mut my_writer);

    println!("{}", String::from_utf8(my_writer.buffer).unwrap());

    Ok(())
}

#[cfg(test)]
mod std_write {
    use super::*;

    #[test]
    fn test_1() {
        // 自前の型のインスタンスを作る
        let mut my_writer = MyWriter { buffer: Vec::new() };
        // 自前の型を標準出力に書き込む関数に渡す
        write_to_stdout2(&mut my_writer);
        // バッファの内容を確認する
        assert_eq!("Hello, world!\n".as_bytes(), my_writer.buffer);
    }
}
