import { createRouter, createWebHashHistory } from "vue-router";

const router: any = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: "",
            name: "DefaultLayout",
            component: () => import("../components/layouts/Default.vue"),
            children: [
                {
                    path: "",
                    name: "Home",
                    component: () => import('../views/Home.vue'),
                }
            ]
        }
    ]
});

// Creates a `nextMiddleware()` function which not only
// runs the default `next()` callback but also triggers
// the subsequent Middleware function.
function nextFactory (context: any, middleware: any, index: number) {
    const subsequentMiddleware = middleware[index];
    // If no subsequent Middleware exists,
    // the default `next()` callback is returned.
    if (!subsequentMiddleware) return context.next;
  
    return (...parameters: any) => {
      // Run the default Vue Router `next()` callback first.
      context.next(...parameters);
      // Then run the subsequent Middleware with a new
      // `nextMiddleware()` callback.
      const nextMiddleware = nextFactory(context, middleware, index + 1);
      subsequentMiddleware({ ...context, next: nextMiddleware });
    };
  }
  
  
  router.beforeEach((to: any, from: any, next: any) => {
  
    //check middleware
    if (to.meta.middleware) {
      const middleware = Array.isArray(to.meta.middleware)
        ? to.meta.middleware
        : [to.meta.middleware];
  
      const context = {
        from,
        next,
        router,
        to,
      };
      const nextMiddleware = nextFactory(context, middleware, 1);
  
      return middleware[0]({ ...context, next: nextMiddleware });
    }
  
    return next()
  })
  
  export default router;
  