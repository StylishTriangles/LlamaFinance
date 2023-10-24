import type { RouteRecordRaw } from "vue-router";
import { createRouter, createWebHistory } from "vue-router";
import NProgress from "~/config/nprogress";

export const basicRoutes = [
  {
    path: "/",
    component: () => import("~/layouts/index.vue"),
    children: [
      {
        path: "",
        name: "Home",
        component: () => import("~/views/Home.vue"),
      },
      {
        path: "/store-test",
        name: "StoreTest",
        component: () => import("~/views/StoreTest.vue"),
      },
    ],
  },
  // Always leave this as last one
  {
    path: "/:pathMatch(.*)*",
    component: () => import("~/layouts/index.vue"),
    children: [
      {
        path: "",
        name: "Not Found",
        component: () => import("~/views/ErrorPage.vue"),
        meta: { title: "404 Not Fount" },
      },
    ],
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes: basicRoutes as unknown as RouteRecordRaw[],
  scrollBehavior: () => ({ left: 0, top: 0 }),
});

// Injection Progress
router.beforeEach(() => {
  if (!NProgress.isStarted()) {
    NProgress.start();
  }
});

router.afterEach(() => {
  NProgress.done();
});
