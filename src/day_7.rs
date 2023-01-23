// https://adventofcode.com/2022/day/7
// Day 7:  No Space Left On Device.
// Given a set of CLI commands (cd, ls, etc), recreate the file structure of a hard drive with 70000000 units of free space.
// In part 1, return the sum of all folder sizes that are less than 100000 (of arbitrary unit)
// In part 2, find the smallest directory to delete that frees up a total of 30000000 units.

use super::*;
use std::{collections::{HashMap, hash_map::Entry}, rc::{Rc, Weak}, cell::RefCell, error, fmt};
use regex::Regex;
use lazy_static::lazy_static;

// A DirectoryNode is one instance of a node in the folder chain. (Follows Newtype pattern)
// It's a wrapper around a DirectoryEntry, which has  shared ownership and has interior mutability, 
// so its parent folder can be accessed  and it can be modified while keeping its structure. 
// Root directory should be kept in scope so that Weak reference to parents are not dropped.
// DirectoryNode semantically represents a tree of files and folders, mimicking the structure of a hard drive.
struct DirectoryNode (Rc<RefCell<DirectoryEntry>>);
type ParentAlias = Weak<RefCell<DirectoryEntry>>;

// a DirectoryEntry, which is either a Folder or a File
enum DirectoryEntry {
    Folder(Option<ParentAlias>, HashMap<String, DirectoryNode>), // Weak ref to parent node, and HashMap of chldren nodes
    File(Option<ParentAlias>, u32) // Weak ref to parent node, and file size
}

// A type of file navigation command
enum ParsedCommand {
    CdIntoFolder(String), // Navigate into subfolder (by String representing the folder name)
    CdOutOfFolder, // navigate to parent
    CdToRoot, // Navigate back to root
    Ls(Vec::<String>), // Add listed entries (in Vec) to structure
}

// Simulated computer information
const TOTAL_SPACE : u32 = 70000000; 
const SPACE_REQUIRED_FOR_UPDATE : u32 = 30000000; 

// Run challenge.
// Main entry point to day 7 challenge.
pub fn run(part_2 : bool) -> Result<(),Box<dyn error::Error>>{
    
    // Extract input into string (newlines kept)
    let f = File::open("input/day7input.txt")?;
    let mut buf = BufReader::new(f);

    let mut input = String::new();
    buf.read_to_string(&mut input)?;

    // Split input into commands along the '$' marker
    let commands : Vec<Result<ParsedCommand, regex::Error>> = input.trim().split('$').filter(|l| l.len() > 0).map(
        |l| {
            ParsedCommand::from_line(l)
    }).collect();

    // Create file structure root
    let root = DirectoryNode::new();

    // Create Rc strong reference to root to perform commands on (keep original root
    // in scope so parent references don't get dropped)
    let mut current_node = root.rc_clone();

    // Iterate over each command and apply it to the current node
    for command in commands {
        let command = command?;
        current_node = current_node.command(command)?;
    }

    let part = if part_2 {2} else {1};

    let size_val;
    if part_2 {

        // Part 2:
        // Calculate minimum folder deletion size to free up enough space for update
        let free_space = TOTAL_SPACE - root.calculate_size();
        let min_deletion_size = SPACE_REQUIRED_FOR_UPDATE - free_space;

        // Fetch size of smallest directory over minimum deletion size
        size_val = root.smallest_directory_size_over_min(min_deletion_size).unwrap();
    } else  {
        // Part 1:
        // Fetch sum of directory sizes for directories under 100000 units
        size_val = root.sum_directory_sizes_under_max(100000);
    }

    println!("Result for day 7-{part} = {size_val}");
    Ok(())
}




impl DirectoryNode {

    // Create new empty root node. This should be kept in scope to ensure no nodes are dropped.
    fn new() -> DirectoryNode {
        DirectoryNode(Rc::new(RefCell::new(DirectoryEntry::Folder(None, HashMap::new()))))
    }

    // Add subfile to node, accessible via key 'name' and of of name String and size 'size'
    fn add_subfile(&self, name: String, size: u32) {

        // Get weak reference to parent node
        let weak_parent = Rc::downgrade( &Rc::clone(&self.0));
        
        // Get shared reference to current entry
        let entry = &Rc::clone(&self.0);
        let mut entry = entry.borrow_mut();

        // Insert subfile as child of current entry
        if let DirectoryEntry::Folder(_,ref mut children) = *entry {
            children.entry(name).or_insert(DirectoryNode(Rc::new(RefCell::new(DirectoryEntry::File(Some(weak_parent), size)))));
        }
    }

    // Add subfolder to node, accessible via key 'name' and with empty children HashMap
    fn add_subfolder(&self, name: String) {
        // Get weak reference to parent node
        let weak_parent = Rc::downgrade( &Rc::clone(&self.0));
        
        // Get shared reference to current entry
        let entry = &Rc::clone(&self.0);
        let mut entry = entry.borrow_mut();

        // Insert subfolder as child of current entry
        if let DirectoryEntry::Folder(_, ref mut children) = *entry {
            children.entry(name).or_insert(DirectoryNode(Rc::new(RefCell::new(DirectoryEntry::Folder(Some(weak_parent), HashMap::new())))));
        }
    }

    // Calculates node total size. 
    // If a file, returns file size, and if a folder, returns all file sizes within folder and subfolderes recursively.
    fn calculate_size(&self) -> u32 {
        let (_,size) = self.get_all_directory_sizes();
        size
    }


    // Get a tuple of:
    // - a Vector of of all directory sizes
    // - the size of this topmost directory or file
    // (This does not include file sizes as elements, only directories, but directory sizes are recursive sum of all files within)
    fn get_all_directory_sizes(&self) -> (Vec<u32>, u32) {

        // Get shared reference to current entry
        let entry = &Rc::clone(&self.0);
        let mut entry = entry.borrow_mut();

        match *entry {
            // If a file, return base case of current file size
            DirectoryEntry::File(_,i) => (Vec::new(),i),

            // If folder, get a Vec of all subdirectory sizes contained within
            DirectoryEntry::Folder(_,ref mut subfolders) => {
                let (mut subfolders_vec, folder_size) = subfolders.iter_mut().map(
                        |(_,b)| 
                        b.get_all_directory_sizes()).fold(
                            (Vec::<u32>::new(),0), 
                        |(acc_vec, acc_size), (new_vec, folder_size)| ([acc_vec, new_vec].concat(),acc_size + folder_size));
                
                // Append current size to list, and return
                subfolders_vec.push(folder_size);
                (subfolders_vec, folder_size)

            }
        }
    }

    // Gets the smallest directory or subdirectory within that is at least 'minimum_size'
    fn smallest_directory_size_over_min(&self, minimum_size: u32) -> Option<u32> {
        let (size_list, _) = self.get_all_directory_sizes();
        size_list.iter().filter(|x| **x > minimum_size).copied().min()
    }

    // Gets sum of all directory sizes with size under 'maximum_size' 
    // (directories and their subdirectories are counted, meaning files can be counted many times)
    fn sum_directory_sizes_under_max(&self, maximum_size : u32) -> u32 {
        let (size_list, _) = self.get_all_directory_sizes();
        size_list.iter().filter(|x| **x < maximum_size).copied().sum()
    }

    // Creates a new DirectoryNode instance with shared ownership of member DirectoryEntry
    fn rc_clone(&self) -> DirectoryNode {
        DirectoryNode(Rc::clone(&self.0))
    }

    // Retrieves new DirectoryNode of child folder by key 'name'
    // New DirectoryNode has shared ownership of internal DirectoryEntry
    fn get_subfolder(&self, name : String) -> Result<DirectoryNode,Box<dyn error::Error>> {

        // Get shared reference to current entry
        let entry = &Rc::clone(&self.0);
        let mut entry = entry.borrow_mut();


        // Confirms this is a folder with subfiles/subfolders and gets reference to 'children' hashmap
        if let DirectoryEntry::Folder(_, ref mut children) = *entry {

            // Searches 'children' for child by name 'name'
            if let Entry::Occupied(subfolder) = children.entry(name) {
                Ok(subfolder.get().rc_clone())
            } else {
                Err(Box::new(DirectoryEntryNotExistError)) // could not find child by that name
            }
        } else {
            Err(Box::new(DirectoryEntryTypeError)) // cannot search for subfolders of a file
        }
    }

    // Retrieves new DirectoryNode of child folder by key 'name'
    // New DirectoryNode has shared ownership of internal DirectoryEntry
    fn get_parent(&self) -> Option<DirectoryNode> {

        // Get shared reference to current entry
        let entry = &Rc::clone(&self.0);
        let mut entry = entry.borrow_mut();

        // Retrieves reference to parent from current entry
        let (DirectoryEntry::Folder(ref mut parent, _) | DirectoryEntry::File(ref mut parent, _)) =  *entry;
        
        // If parent exists and has not been dropped, get parent as node
        if let Some(p) = parent {
            if let Some(p) = p.upgrade() {
                return Some(DirectoryNode(p))
            }
        }
        None
    }

    // Retrieves new DirectoryNode of child folder by key 'name'
    // New DirectoryNode has shared ownership of internal DirectoryEntry
    fn get_root(&self) -> DirectoryNode {
        let mut root : DirectoryNode = self.rc_clone();
        while let Some(r) = root.get_parent() {
            root = r;
        }
        root
    }

    // Creates a folder or file within Node based on line 'line'
    // Line is of one of two formats:
    // "dir name" where name is the name, representing a folder/directory
    // "filesize name", where filesize is the size and name is the name, representing a file.
    fn parse_line_to_directoryentry(& self, line: &str) -> Result<(), regex::Error> {

        let line = line.trim();

        // Create directory from:
        // "dir name" (ie: dir filedir)
        lazy_static! {
            static ref REGEX_DIRECTORYENTRY_FOLDER: Regex = Regex::new(r"^dir\s([\w\.]+)$").unwrap();
        }
        if let Some(matches) = REGEX_DIRECTORYENTRY_FOLDER.captures(line)  {
            if let Some(name) = matches.get(1) {
                self.add_subfolder(name.as_str().to_string());
                return Ok(());
            } 
        }
        // Create file from:
        // "filesize name" (ie: 231232 filetxt)
        lazy_static! {
            static ref REGEX_DIRECTORYENTRY_FILE: Regex = Regex::new(r"^(\d+)\s([\w\.]+)$").unwrap();
        }
        if let Some(matches) = REGEX_DIRECTORYENTRY_FILE.captures(line)  {
            if let (Some(size), Some(name) )= (matches.get(1), matches.get(2)) {
                self.add_subfile(name.as_str().to_string(), size.as_str().parse().unwrap()); // unwrap here as it must be digits
                return Ok(());
            } 
        }

        // Could not match command to file format or folder format
        Err(regex::Error::Syntax(format!("could not match DirectoryEntry to any regex syntax: {}",line)))
        
    }

    // Run a ParsedCommand on the current node
    // Returns the new DirectoryNode (or current one if applicable) or an Error
    // let node = node.command(command); 
    fn command(&self, command : ParsedCommand) -> Result<DirectoryNode,Box<dyn error::Error>> {
        let node = self.rc_clone();
        let node = match command {
            // Return subfolder
            ParsedCommand::CdIntoFolder(folder_name) => node.get_subfolder(folder_name)?,

            // Return parent folder
            ParsedCommand::CdOutOfFolder => if let Some(p) = node.get_parent() {p} else {node} ,

            // Return root folder
            ParsedCommand::CdToRoot => node.get_root(),

            // Return same folder, but add directoryentries based on associated Vector
            ParsedCommand::Ls(files) => {
                for line in files {
                    node.parse_line_to_directoryentry(&line.trim())?;        
                }
                node
            }
        };
        Ok(node)    
    }
    
    
}


impl ParsedCommand {

    // Convert a string slice input to a Parsed Command
    // Strings can be of several formats:
    // cd /
    // cd ..
    // cd somename
    // ls <- (and then several directory entry strings separated by newlines)
    fn from_line(l : &str) -> Result<ParsedCommand, regex::Error> {
        let l = l.trim();
        // cd /
        lazy_static! {
            static ref REGEX_COMMAND_CDROOT: Regex = Regex::new(r"^cd /").unwrap();
        }
        if REGEX_COMMAND_CDROOT.is_match(l) {
            return Ok(ParsedCommand::CdToRoot);
        }

        // cd ..
        lazy_static! {
            static ref REGEX_COMMAND_CDPARENT: Regex = Regex::new(r"^cd \.\.").unwrap();
        }
        if REGEX_COMMAND_CDPARENT.is_match(l) {
            return Ok(ParsedCommand::CdOutOfFolder);
        }

        // cd into folder:
        // cd foldername
        lazy_static! {
            static ref REGEX_COMMAND_CDINTO: Regex = Regex::new(r"^cd\s(\w+)").unwrap();
        }
        if REGEX_COMMAND_CDINTO.is_match(l) {
            return Ok(ParsedCommand::CdIntoFolder(l[3..].trim().to_string()));
        }
        // ls
        // found file name
        // found file name
        // found file name
        lazy_static! {
            static ref REGEX_COMMAND_LS: Regex = Regex::new(r"^ls.*").unwrap();
        }
        if REGEX_COMMAND_LS.is_match(l) {
            return Ok(ParsedCommand::Ls(l[3..].trim().split('\n').map(|s| s.trim().to_string()).collect()));
        }

        Err(regex::Error::Syntax(format!("could not match command to any regex syntax: \"{}\"",l)))
    }
}



#[derive(Clone, Debug)]
struct DirectoryEntryTypeError;
impl error::Error for DirectoryEntryTypeError {}
impl fmt::Display for DirectoryEntryTypeError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "performed operation on wrong type of DirectoryEntry (file vs folder)")
    }
}

#[derive(Clone, Debug)]
struct DirectoryEntryNotExistError;
impl error::Error for DirectoryEntryNotExistError {}
impl fmt::Display for DirectoryEntryNotExistError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "attempted to access non-existent entry")
    }
}

#[cfg(test)] 
mod tests {

    use super::*;

    #[test]
    fn smallest_folder_over_minimum() {
        // Create sample directory
        // root
        // -- file_1 500
        // -- file_2 250
        // -- folder_1
        // ----- file_1_1 100
        // ----- file_1_2 350
        // ----- folder_2
        // ------- file_2_1 425
        // ------- file_2_2 600
        // ----- folder_3
        // ------- file_3_1 5
        // ------- file_3_2 5

        let root = DirectoryNode::new();
        root.add_subfile("file_1".to_string(), 500);
        root.add_subfile("file_2".to_string(), 250);
        root.add_subfolder("folder_1".to_string());

        let folder_1 = root.get_subfolder("folder_1".to_string()).unwrap();
        folder_1.add_subfile("file_1_1".to_string(), 100);
        folder_1.add_subfile("file_1_2".to_string(), 350);
        folder_1.add_subfolder("folder_2".to_string());
        folder_1.add_subfolder("folder_3".to_string());

        let folder_2 = folder_1.get_subfolder("folder_2".to_string()).unwrap();
        folder_2.add_subfile("file_2_1".to_string(), 425);
        folder_2.add_subfile("file_2_2".to_string(), 600);

        let folder_3 = folder_1.get_subfolder("folder_3".to_string()).unwrap();
        folder_3.add_subfile("file_3_1".to_string(), 5);
        folder_3.add_subfile("file_3_2".to_string(), 5);


        // Sanity traits about sample directory
        assert_eq!(root.calculate_size(), 2235); // size is 2235
        assert_eq!(root.sum_directory_sizes_under_max(650), 10); // Total size under 650 is 10
        assert_eq!(root.sum_directory_sizes_under_max(1500), 10 + 1025 + 10+1025+100+350); 
        assert_eq!(root.sum_directory_sizes_under_max(99), 10); 
        assert_eq!(root.smallest_directory_size_over_min(6).unwrap(), 10); // Smallest diretory over minimum 6 is 10
        assert_eq!(root.smallest_directory_size_over_min(400).unwrap(), 1025);
        assert_eq!(root.smallest_directory_size_over_min(4).unwrap(), 10);

    }

    #[test]
    fn simple_folder_creation() {

        // Create root directory with two folders in it
        // root
        // -- file_1 500
        // -- file_2 250
        let root = DirectoryNode::new();
        root.add_subfile("file_1".to_string(), 500);
        root.add_subfile("file_2".to_string(), 250);
        assert_eq!(root.calculate_size(), 750);

        // Create folder_1 directory with two folders in it
        // folder_1
        // -- file_1_1 100
        root.add_subfolder("folder_1".to_string());
        let folder_1 = root.get_subfolder("folder_1".to_string()).unwrap();
        folder_1.add_subfile("file_1_1".to_string(), 100);
        assert_eq!(folder_1.calculate_size(), 100);
        assert_eq!(root.calculate_size(), 850);

        // Add a second, empty folder
        // root
        // -- file_1 500
        // -- file_2 250
        // -- folder_1
        // ---- file_1_1 100
        // -- folder_2
        root.add_subfolder("folder_2".to_string());
        assert_eq!(folder_1.calculate_size(), 100);

    }

    #[test]
    fn parse_input_into_directory() {
        // Create root directory with two example files in it from challenge
        let root = DirectoryNode::new();
        root.parse_line_to_directoryentry("290229 dsm").unwrap();
        root.parse_line_to_directoryentry("273438 fsjwz.css").unwrap();
        assert_eq!(root.calculate_size(), 290229+273438);

        // Create subfolder, and put file in it
        root.parse_line_to_directoryentry("dir test_folder").unwrap();
        let test_folder = root.get_subfolder("test_folder".to_string()).unwrap();
        test_folder.parse_line_to_directoryentry("100000 fsjwz.css").unwrap();
        assert_eq!(root.calculate_size(), 290229+273438 + 100000);
    }

    #[test]
    fn parse_run_commands() {
        // Tests parsing of commands and running those commands to ensure final filesystem is as expected and 
        // recreateable from string commands.

        // Create root directory with two example files in it from challenge
        let root_original = DirectoryNode::new();

        // Run simple ls command to create file and a subfolder
        let node = root_original.command(ParsedCommand::from_line(
            "ls 
            290229 dsm
            dir folder1
            273438 fsjwz12321.css").unwrap()).unwrap();
        assert_eq!(node.calculate_size(), 290229+273438);

        // Enter subfolder and create further subentries
        let node = node.command( ParsedCommand::from_line(
            "cd folder1").unwrap()).unwrap();
        let node = node.command( ParsedCommand::from_line(
            "ls 
            dir folder2
            100000 fsjwz.css").unwrap()).unwrap();
        assert_eq!(node.calculate_size(), 100000);

        // Return to parent
        let node = node.command( ParsedCommand::from_line(
            "cd ..").unwrap()).unwrap();
        assert_eq!(node.calculate_size(), 290229+273438+100000);

        // Enter fodler all the way in, then resset to root
        let node = node.command(ParsedCommand::from_line(
            "cd folder1").unwrap()).unwrap();
            let node = node.command(ParsedCommand::from_line(
                "cd folder2").unwrap()).unwrap();
            assert_eq!(node.calculate_size(), 0);

        let node = node.command( ParsedCommand::from_line(
            "cd /").unwrap()).unwrap();
        assert_eq!(node.calculate_size(), 290229+273438+100000);
        
    }
}