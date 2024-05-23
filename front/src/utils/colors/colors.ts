import { ColorMode } from "../../types/calendar";
import { ShortSessionInfo } from "../../types/core";
import darkModeColors from "./darkModeColors";
import lightModeColors from "./lightModeColors";

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
  private currentIndex: number;

  public constructor(private colors: string[]) {
    this.currentIndex = 0;
  }

  public next(): string {
    const color = this.colors[this.currentIndex];
    this.currentIndex = (this.currentIndex + 1) % this.colors.length;
    return color;
  }
}

export function getColorBySessionId(
  sessions: ShortSessionInfo[],
  colorMode: ColorMode | null,
  darkMode: boolean
): Record<string, string> {
  if (colorMode === null) {
    return {};
  }
  const calendarColorIterator = new ColorIterator(
    darkMode ? darkModeColors : lightModeColors
  );

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
