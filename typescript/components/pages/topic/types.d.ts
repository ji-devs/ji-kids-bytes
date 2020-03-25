type TopicManifest = {
  meta: TopicMeta,
  videos: Array<Media>,
  games: Array<Media>,
  discovers: Array<Link>,
  create: Create,
  crafts: Array<Link>,
}

type TopicMeta = {
  id: string,
  title: string,
  locked: boolean,
}

type Media = {
  id: String,
  player: MediaPlayer
}

declare enum MediaPlayer {
  Youtube = "youtube",
  JiTap = "jitap"
}

type Link = {
  link: string,
  link_label?: string,

  image_filename: string,

  title: string,
}

type Create = {
  tool: CreationTool,

  image_filename: string,

  title: string,

  body: string,
}
declare enum CreationTool {
  JiTap = "jitap",
  JiStudio = "jistudio"
}
