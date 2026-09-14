use crate::{
    data::cs::{
        file::{File, Location, SortedFiles},
        group::Group,
        localization::{Locale, LocalizationObject, LocalizedObject},
        object::{Key, Object, SortedObjects},
    },
    error::Error,
};
use std::{collections::HashMap, ops::Range, path::Path};

pub(crate) mod file;
pub(crate) mod group;
pub(crate) mod localization;
pub(crate) mod object;
pub(crate) mod text;

#[derive(Debug)]
pub(crate) struct Data {
    core: SortedFiles,
    localization: HashMap<Locale, Vec<File>>,
}

impl Data {
    pub(crate) fn load<P: AsRef<Path>>(base_dir: P) -> Result<Self, Error> {
        let cs_dir = base_dir.as_ref().join("content/cs");

        let core = {
            let path = cs_dir.join("core");
            let files = File::load(path)?;
            SortedFiles::new(files)?
        };
        let localization = Locale::ALL
            .into_iter()
            .map(|locale| {
                let path = cs_dir.join(locale.folder());
                let files = File::load(path)?;
                Ok((locale, files))
            })
            .collect::<Result<_, Error>>()?;
        Ok(Data { core, localization })
    }

    pub(crate) fn view<'a>(&'a self) -> Result<DataView<'a>, Error> {
        let (cores, file_objects) = {
            let mut core = Vec::new();
            let mut map: HashMap<&Location, Range<usize>> = HashMap::new();
            for file in &*self.core {
                let objects = SortedObjects::new(Object::from_file(file)?)?;

                let start = core.len();
                core.extend(objects);
                let end = core.len();
                map.insert(file.location(), start..end);
            }
            (core, map)
        };
        let objects = {
            let localizations: Vec<LocalizationObject> = self
                .localization
                .iter()
                .map(|(locale, files)| {
                    let objects: Vec<LocalizationObject> = Object::from_files(files)?
                        .into_iter()
                        .map(|object| LocalizationObject::new(*locale, object))
                        .collect();
                    Ok(objects)
                })
                .collect::<Result<Vec<_>, Error>>()?
                .into_iter()
                .flatten()
                .collect();
            LocalizedObject::from_objects(cores, localizations)?
        };
        let group_files = {
            let mut map: HashMap<Group, Vec<&File>> = HashMap::new();
            for file in &*self.core {
                let group = file.group()?.parse()?;
                map.entry(group).or_default().push(file);
            }
            map
        };
        let key_objects = {
            objects
                .iter()
                .enumerate()
                .map(|(index, object)| Ok((object.key()?, index)))
                .collect::<Result<HashMap<_, _>, Error>>()?
        };

        Ok(DataView {
            objects,
            group_files,
            file_objects,
            key_objects,
        })
    }
}

#[derive(Debug)]
pub(crate) struct DataView<'a> {
    objects: Vec<LocalizedObject<'a, 'a>>,
    group_files: HashMap<Group, Vec<&'a File>>,
    file_objects: HashMap<&'a Location, Range<usize>>,
    key_objects: HashMap<Key<'a>, usize>,
}

impl<'a> DataView<'a> {
    pub(crate) fn objects(&self) -> &Vec<LocalizedObject<'a, 'a>> {
        &self.objects
    }

    pub(crate) fn index(&self, key: &Key) -> Option<&LocalizedObject<'a, 'a>> {
        let index = **&self.key_objects.get(key)?;
        Some(&self.objects[index])
    }

    pub(crate) fn file_locations(&self, group: Group) -> Vec<&'a Location> {
        self.group_files[&group]
            .iter()
            .map(|file| file.location())
            .collect()
    }

    pub(crate) fn location_objects(&self, location: &Location) -> Vec<&LocalizedObject<'a, 'a>> {
        self.file_objects[location]
            .clone()
            .into_iter()
            .map(|index| &self.objects[index])
            .collect()
    }
}
