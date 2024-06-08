use backlog::project::ProjectBuilder;
fn main() {
    let project = ProjectBuilder::new()
        .id(1)
        .key("KEY")
        .name("Name")
        .description("Description")
        .build();
    println!("{:?}", project);
}
