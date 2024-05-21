import { FC } from "react";
import { useParams } from "react-router-dom";
import NewCalendarPage from "./NewCalendar";
import CalendarPage from "./CalendarPage";

const CalendarHome: FC = () => {
  const { solutionId } = useParams<"solutionId">();

  return solutionId === undefined ? (
    <NewCalendarPage />
  ) : (
    <CalendarPage solutionId={solutionId} />
  );
};

export default CalendarHome;
