import { createRouter, createWebHashHistory } from "vue-router";
import ChapterList from "../components/library/ChapterList.vue";
import ChapterView from "../components/library/ChapterView.vue";
import LibraryView from "../components/library/LibraryView.vue";

const routes = [
  {
    path: "/",
    name: "LibraryView",
    component: LibraryView,
  },
  {
    path: "/chapters/:series",
    name: "chapters",
    component: ChapterList,
  },
  {
    path: "/view/:series/:chapter",
    name: "view",
    component: ChapterView,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
