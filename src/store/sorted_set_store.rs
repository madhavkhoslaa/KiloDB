use crate::traits::Store::Store;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct SortedSetStore {
    // Using BTreeMap for efficient range queries
    // Key: score, Value: Vec of members with that score
    scores: BTreeMap<String, f64>, // member -> score mapping
    members_by_score: BTreeMap<i64, Vec<String>>, // score_as_int -> members (for ordering)
}

impl SortedSetStore {
    pub fn new() -> Self {
        SortedSetStore {
            scores: BTreeMap::new(),
            members_by_score: BTreeMap::new(),
        }
    }

    pub fn add_member(&mut self, member: &str, score: f64) -> bool {
        let score_key = (score * 1000000.0) as i64; // Convert to int for ordering
        let was_new = !self.scores.contains_key(member);
        
        // Remove old entry if exists
        if let Some(old_score) = self.scores.get(member) {
            let old_score_key = (*old_score * 1000000.0) as i64;
            if let Some(members) = self.members_by_score.get_mut(&old_score_key) {
                members.retain(|m| m != member);
                if members.is_empty() {
                    self.members_by_score.remove(&old_score_key);
                }
            }
        }

        // Add new entry
        self.scores.insert(member.to_string(), score);
        self.members_by_score
            .entry(score_key)
            .or_insert_with(Vec::new)
            .push(member.to_string());

        was_new
    }

    pub fn remove_member(&mut self, member: &str) -> bool {
        if let Some(score) = self.scores.remove(member) {
            let score_key = (score * 1000000.0) as i64;
            if let Some(members) = self.members_by_score.get_mut(&score_key) {
                members.retain(|m| m != member);
                if members.is_empty() {
                    self.members_by_score.remove(&score_key);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_score(&self, member: &str) -> Option<f64> {
        self.scores.get(member).copied()
    }

    pub fn get_range(&self, start: isize, stop: isize) -> Vec<String> {
        let all_members: Vec<String> = self.members_by_score
            .values()
            .flat_map(|v| v.iter())
            .cloned()
            .collect();
        
        let len = all_members.len() as isize;
        let start_idx = if start < 0 { (len + start).max(0) as usize } else { start.min(len) as usize };
        let stop_idx = if stop < 0 { (len + stop + 1).max(0) as usize } else { (stop + 1).min(len) as usize };
        
        all_members[start_idx..stop_idx].to_vec()
    }

    pub fn get_rank(&self, member: &str) -> Option<usize> {
        if !self.scores.contains_key(member) {
            return None;
        }

        let all_members: Vec<String> = self.members_by_score
            .values()
            .flat_map(|v| v.iter())
            .cloned()
            .collect();
        
        all_members.iter().position(|m| m == member)
    }

    pub fn len(&self) -> usize {
        self.scores.len()
    }
}

impl Store for SortedSetStore {} 