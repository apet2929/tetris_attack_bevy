use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run()
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(people_without_jobs)
            .add_system(person_does_job);
    }
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name)
    }
}

pub fn people_with_jobs(
    person_query: Query<&Person, With<Employed>>
) {
    for person in person_query.iter() {
        println!("Name: {} has a job.", person.name)
    }
}

pub fn people_without_jobs(
    person_query: Query<&Person, Without<Employed>>
) {
    for person in person_query.iter() {
        println!("Name: {} has no job.", person.name)
    }
}

pub fn person_does_job(
    person_query: Query<(&Person, &Employed)>
) {
    for(person, job) in person_query.iter(){
        println!("Name {} has job {:?}", person.name, job.job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (Person {
            name: "Alex".to_string()
        },
         Employed {
             job: Job::Lawyer
         }
        ));

    commands.spawn(
        (Person {
            name: "John".to_string()
        },
         Employed {
             job: Job::FireFighter
         }
        ));

    commands.spawn(
        (Person {
            name: "Felix".to_string()
        }
        ));

    commands.spawn(
        (Person {
            name: "Andrew".to_string()
        },
         Employed {
             job: Job::Doctor
         }
        ));

}

pub fn hello_world() {
    println!("Hello world!");
}

#[derive(Component)]
pub struct Person {
    pub name: String
}

#[derive(Component)]
pub struct Employed {
    pub job: Job
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}