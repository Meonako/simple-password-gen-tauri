<script lang="ts">
    import "../app.css";
    import Titlebar from "$lib/Titlebar.svelte";
</script>

<main class="container">
    <Titlebar title="Password Generator" />
    <slot>
        <h1>Unexpected Error</h1>
    </slot>
</main>
