type TopicManifest = {
  meta: TopicMeta,
  videos: Array<Video>,
  games: Array<Game>,
  discovers: Array<Discover>,
  creates: Array<Create>,
  crafts: Array<Craft>,
}

type TopicMeta = {
  id: string,
  title: string,
}

type Video = {
  id: string,
}

type Game = {
  id: string,
}

type Discover = {
  link: string,

  image_filename: string,

  title: string,

  desc: string,
}

type Create = {
  link: string,

  image_filename: string,

  header: string,

  body: string,
}

type Craft = {
  link: string,

  image_filename: string,

  header: string,

  body: string,
}