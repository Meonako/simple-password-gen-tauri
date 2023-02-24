<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let password = "";
    let passwordLength = 10;
    let generated = false;

    async function generate() {
        generated = true
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        password = await invoke("generate", { passwordLength });
    }
</script>

<div>
    <span>Password length: </span>
    <input type="number" placeholder="How long should your password be?" bind:value={passwordLength}>
    <button on:click={() => generate()}>Generate</button>
    <div class="row">
        <input
            placeholder="Password goes here..."
            bind:value={password}
        />
    </div>
    {#if generated}
        <p>Now try changing Password length to 100!</p>
    {/if}
</div>