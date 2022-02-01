<script>
  import { operationStore, query, createClient, setClient, defaultExchanges, dedupExchange, fetchExchange, cacheExchange } from '@urql/svelte';

  const client = createClient({
	url: "http://localhost:8000/graphql",
	exchanges: [
      dedupExchange,
      cacheExchange,
      fetchExchange
    ]
  });

  setClient(client);

  const todos = operationStore(`
    query {
 	value
}`);

  query(todos);
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

value: {$todos.data && $todos.data.value}
