<script>
    export let data;
    $: ({title, is_completed, id} = data);
</script>

<h1>Welcome to SvelteKit</h1>
<p>title: {title}</p>
<p>is_completed: {is_completed}</p>
<p>id: {id}</p>
