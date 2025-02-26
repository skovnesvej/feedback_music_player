// Imports
use std::{fs, io};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use rodio::{Decoder, OutputStream, Sink};


fn create_playlist(folder_name: &str) -> io::Result<()> {
    let path = format!("./playlist/{}", folder_name);
    fs::create_dir_all(&path)?; // Creates folder
    println!("Playlist '{}' created successfully!", folder_name);
    Ok(())
}


fn add_music_to_playlist(folder_name: &str, file_name: &str) -> io::Result<()> {
    let music_file = Path::new("music").join(file_name);
    let folder = Path::new("playlist").join(folder_name);
    let destination_file = folder.join(file_name);

    if !music_file.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Music file does not exist"));
    }
    if !folder.exists() {
        fs::create_dir_all(&folder)?; // Creates a folder if it does not exist
    }

    fs::copy(&music_file, &destination_file)?; // Copy the music file to the folder
    println!("File '{}' copied to '{}'", file_name, folder.display());
    Ok(())
}


fn play_song(folder_name: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("playlist").join(folder_name).join(file_name);

    // Check if file exists and return an error if not
    if !file_path.exists() {
        return Err("MP3 file not found".into());
    }

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?; // Sink controls the music playback
    let file = File::open(&file_path)?; // Open music file
    let source = Decoder::new(BufReader::new(file))?;

    sink.append(source);
    sink.sleep_until_end(); // Makes the program wait until the music playback is done

    println!("Playing: {:?}", file_path);
    Ok(())
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    loop {
        println!("\nChoose an option:");
        println!("1. Create Playlist");
        println!("2. Add Music to Playlist");
        println!("3. Play Music from Playlist");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let folder_name = get_user_input("Enter the playlist name: ");
                if let Err(e) = create_playlist(&folder_name) {
                    eprintln!("Error: {}", e);
                }
            }
            "2" => {
                let folder_name = get_user_input("Enter the playlist name: ");
                let file_name = get_user_input("Enter the music file name: ");
                if let Err(e) = add_music_to_playlist(&folder_name, &file_name) {
                    eprintln!("Error: {}", e);
                }
            }
            "3" => {
                let folder_name = get_user_input("Enter the playlist (folder) name: ");
                let file_name = get_user_input("Enter the music file name: ");
                if let Err(e) = play_song(&folder_name, &file_name) {
                    eprintln!("Error: {}", e);
                }
            }
            "4" => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 4."), // Default choice if input is not found
        }
    }
}
