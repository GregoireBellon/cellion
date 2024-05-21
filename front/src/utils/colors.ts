import { ColorMode } from "../types/calendar";
import { ShortSessionInfo } from "../types/core";

function ColorModeKeyGetter(
  colorMode: ColorMode
): (sessions: ShortSessionInfo) => string {
  if (colorMode === ColorMode.BY_COURSE) {
    return (session: ShortSessionInfo) => session.course.id;
  } else if (colorMode === ColorMode.BY_PART) {
    return (session: ShortSessionInfo) => session.part.id;
  } else if (colorMode === ColorMode.BY_ROOM) {
    return (session: ShortSessionInfo) => session.rooms[0].id;
  } else {
    return (session: ShortSessionInfo) => session.teachers[0].id;
  }
}

export class ColorIterator {
  private static ALL_COLORS: string[] = [
    "#0000FF",
    "#FF4500",
    "#8A2BE2",
    "#7FFF00",
    "#DC143C",
    "#00CED1",
    "#FFD700",
    "#00FF7F",
    "#DAA520",
    "#008B8B",
    "#4682B4",
    "#32CD32",
    "#FF6347",
    "#40E0D0",
    "#7FFFD4",
    "#8B008B",
    "#00FA9A",
    "#B8860B",
    "#9932CC",
    "#3CB371",
    "#F08080",
    "#20B2AA",
    "#FFA07A",
    "#6A5ACD",
    "#ADFF2F",
    "#BA55D3",
    "#2E8B57",
    "#87CEEB",
    "#F0E68C",
    "#9400D3",
    "#48D1CC",
    "#D2691E",
    "#6495ED",
    "#FA8072",
    "#5F9EA0",
    "#FF69B4",
    "#66CDAA",
    "#F5DEB3",
    "#87CEFA",
    "#FF4500",
    "#4682B4",
    "#B0C4DE",
    "#FFD700",
    "#AFEEEE",
    "#8A2BE2",
    "#32CD32",
    "#FF6347",
    "#000080",
    "#8B4513",
    "#4169E1",
    "#008080",
    "#0000CD",
    "#808000",
    "#FFA500",
    "#800000",
    "#00FFFF",
    "#808080",
    "#FFFF00",
    "#800080",
    "#008000",
    "#800080",
    "#A52A2A",
    "#696969",
    "#8B008B",
    "#FFA07A",
    "#808000",
    "#FFC0CB",
    "#FFD700",
    "#2F4F4F",
    "#CD853F",
    "#D2691E",
    "#FF4500",
    "#778899",
    "#DAA520",
    "#8B0000",
    "#006400",
    "#B22222",
    "#2E8B57",
    "#9400D3",
    "#8B4513",
    "#556B2F",
    "#A0522D",
    "#0000FF",
    "#FF6347",
    "#708090",
    "#CD5C5C",
    "#696969",
    "#DC143C",
    "#FFA500",
    "#808080",
    "#FF1493",
    "#CD5C5C",
    "#8B0000",
    "#008080",
    "#4B0082",
    "#2F4F4F",
    "#8B4513",
    "#FFD700",
    "#A0522D",
    "#4682B4",
    "#DAA520",
    "#696969",
  ];

  private currentIndex: number;

  public constructor() {
    this.currentIndex = 0;
  }

  public next(): string {
    const color = ColorIterator.ALL_COLORS[this.currentIndex];
    this.currentIndex =
      (this.currentIndex + 1) % ColorIterator.ALL_COLORS.length;
    return color;
  }
}

export function getColorBySessionId(
  sessions: ShortSessionInfo[],
  colorMode: ColorMode | null
): Record<string, string> {
  if (colorMode === null) {
    return {};
  }
  const calendarColorIterator = new ColorIterator();

  const getKey = ColorModeKeyGetter(colorMode);
  const colorByCriterion = sessions.reduce(
    (acc: Record<string, { color: string; ids: string[] }>, current) => {
      const key = getKey(current);

      if (acc[key] === undefined) {
        acc[key] = {
          color: calendarColorIterator.next(),
          ids: [current.id],
        };
      } else {
        acc[key].ids.push(current.id);
      }

      return acc;
    },
    {}
  );
  return Object.fromEntries(
    Object.values(colorByCriterion).flatMap((v) =>
      v.ids.map((id) => [id, v.color])
    )
  );
}
