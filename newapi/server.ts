import { initTRPC } from '@trpc/server';
import { OpenApiMeta } from "trpc-openapi"
 
/**
 * Initialization of tRPC backend
 * Should be done only once per backend!
 */
const t = initTRPC.meta<OpenApiMeta>().create();
 
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
    .meta({ openapi: { method: 'GET', path: '/user' } })
    .input(z.object({ id: z.string() }))
    .output(z.object({ id: z.string(), name: z.string() }))
    .query(async (opts) => {
      const { input } = opts;
      const user = await db.user.findById(input.id);
      return user;
    }),
});

import { generateOpenApiDocument } from 'trpc-openapi';

const doc = generateOpenApiDocument(appRouter, {
  title: 'My API',
  version: '0.1.0',
  description: 'My API description',
  baseUrl: 'http://localhost:3000',
});
console.log(JSON.stringify(doc, null, 2));

// Export type router type signature,
// NOT the router itself.
export type AppRouter = typeof appRouter;

import { createHTTPServer } from '@trpc/server/adapters/standalone';

const server = createHTTPServer({
  router: appRouter,
});

server.listen(3000);
