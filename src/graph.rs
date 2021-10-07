use std::collections::HashMap;

use generational_arena::Index;

use crate::note::Note;
use crate::notes::Notes;

pub fn get_connected_component_undirected<'a>(notes: &'a Notes, note: &'a Note) -> Vec<&'a Note> {
    let mut visited = HashMap::new();
    for (i, n) in notes.index_iter() {
        visited.insert(i, n.filename == note.filename);
    }
    depth_first_search(&mut visited, notes, note, 0);
    notes
        .index_iter()
        .filter(|(i, _)| visited[i])
        .map(|(_, note)| note)
        .collect()
}

fn depth_first_search<'a, 'b>(
    visited: &'b mut HashMap<Index, bool>,
    notes: &'a Notes,
    note: &'a Note,
    level: i32,
) {
    for index in note.links.iter().chain(note.backlinks.iter()) {
        if !visited[index] {
            visited.insert(*index, true);
            depth_first_search(visited, notes, &notes[*index], level + 1);
        }
    }
}
