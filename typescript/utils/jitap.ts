export const gameIdToAlbumId = (gameId:string):string => 
    (parseInt(gameId.substring(1), 36) - 1234).toString();