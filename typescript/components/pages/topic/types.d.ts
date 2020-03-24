type TopicManifest = {
  meta: TopicMeta,
  videos: Array<Video>,
  games: Array<Game>,
  discovers: Array<Discover>,
  create: Create,
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
  link_label: string,

  image_filename: string,

  title: string,

  desc: string,
}

declare enum CreationTool {
  JiTap = "jitap",
  JiStudio = "jistudio"
}

type Create = {
  tool: CreationTool,

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