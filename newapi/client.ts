import fetch from 'node-fetch';
// @ts-expect-error
global.fetch = fetch;
import { createTRPCProxyClient, httpBatchLink } from '@trpc/client';
import type { AppRouter } from './server';

const trpc = createTRPCProxyClient<AppRouter>({
  links: [httpBatchLink({ url: 'http://localhost:3000' })],
});

(async () => {
  const user = await trpc.userById.query('1');
  console.log('user', user);
})();
