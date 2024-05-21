import React from "react";
import ReactDOM from "react-dom/client";
import App from "./components/App.tsx";
import {
  createBrowserRouter,
  Navigate,
  RouterProvider,
} from "react-router-dom";
import FilesPage from "./components/FilesPage.tsx";
import CalendarHome from "./components/CalendarPage/CalendarHome.tsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    children: [
      { path: "", element: <Navigate replace to="calendar" /> },
      {
        path: "calendar",
        element: <CalendarHome />,
      },
      {
        path: "calendar/:solutionId",
        element: <CalendarHome />,
      },
      {
        path: "files",
        element: <FilesPage />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
