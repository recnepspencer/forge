use worth_relational::facade::transactions::CommitResult;

fn cannot_substitute_created_entity_mapping(target: &mut CommitResult, other: &CommitResult) {
    target.created_entities = other.created_entities.clone();
}

fn main() {}
