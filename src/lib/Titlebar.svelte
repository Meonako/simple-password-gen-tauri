<script>
    export let title = "Application";

    import { appWindow } from "@tauri-apps/api/window";
</script>

<div data-tauri-drag-region class="titlebar">
    <p data-tauri-drag-region id="title-header">{title}</p>
    <div class="titlebar-button-container">
        <button class="titlebar-button" on:click={() => appWindow.minimize()}>
            <img
                src="https://api.iconify.design/mdi:window-minimize.svg"
                alt="minimize"
            />
        </button>
        <button
            class="titlebar-button"
            on:click={() => appWindow.toggleMaximize()}
        >
            <img
                src="https://api.iconify.design/mdi:window-maximize.svg"
                alt="maximize"
            />
        </button>
        <button class="titlebar-button" on:click={() => appWindow.close()}>
            <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </button>
    </div>
</div>

<style>
    #title-header {
        margin: 0 auto;
        text-align: center;
        width: 100%;
    }

    .titlebar {
        height: 30px;
        user-select: none;
        display: flex;
        justify-content: center;
        align-items: center;
        text-align: center;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        border-bottom: 1px solid rgba(100, 100, 100, 0.8);
    }

    .titlebar-button-container {
        display: flex;
        justify-content: flex-end;
    }

    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 30px;
        height: 30px;
        border-radius: 0;
        transition: 0.5s;
    }

    .titlebar-button:hover {
        background: #363636;
    }
</style>
