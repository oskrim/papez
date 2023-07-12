// type User = { id: string; name: string; };

// userList: () => User[];
// userById: (id: string) => User;
// userCreate: (data: { name: string }) => User;

import { initTRPC } from '@trpc/server';
 
/**
 * Initialization of tRPC backend
 * Should be done only once per backend!
 */
const t = initTRPC.create();
 
/**
 * Export reusable router and procedure helpers
 * that can be used throughout the router
 */
export const router = t.router;
export const publicProcedure = t.procedure;

const db = {
  user: {
    findMany: async () => {
      return [{ id: '1', name: 'Alice' }];
    },
    findById: async (id: string) => {
      return { id, name: 'Alice' };
    },
  }
};

import { z } from 'zod';

const appRouter = router({
  userById: publicProcedure
    .input(z.string())
    .query(async (opts) => {
      const { input } = opts;
      const user = await db.user.findById(input);
      return user;
    }),
});

// Export type router type signature,
// NOT the router itself.
export type AppRouter = typeof appRouter;

import { createHTTPServer } from '@trpc/server/adapters/standalone';

const server = createHTTPServer({
  router: appRouter,
});

server.listen(3000);
