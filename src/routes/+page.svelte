<script>
    import { invoke } from '@tauri-apps/api/core';

    let title_text = "";
    let content_text = "";
</script>

<div class="w-full min-h-screen bg-zinc-900 text-white flex flex-col flex-grow">
    <input type="text" class="w-full h-1/6 bg-zinc-900 p-10 resize-none text-center text-4xl" placeholder="Note Title" bind:value={title_text}>
    <textarea name="" id="" class="w-full h-5/6 px-[20rem] py-10 mx-auto appearance-none grow resize-none bg-zinc-900" placeholder="Note Body" bind:value={content_text}></textarea>
    <button class="w-[5rem] h-[2rem] fixed bottom-2 right-2 m-2 bg-orange-400 rounded hover:bg-orange-600" on:click={invoke('save', { title: title_text})}>Save</button>
    <label for="openFile" class="w-[5rem] h-[2rem] bg-orange-400 fixed bottom-2 right-24 m-2 rounded text-center pt-1 hover:bg-orange-600">Open</label>
    <input type="file" id="openFile" placeholder="" class="hidden">
    
    
</div>

<style>
    textarea {
        @apply border-none outline-none;
    }

    textarea:focus {
        @apply border-none outline-none appearance-none;
    }
    svg:hover {
        @apply scale-105;
    }

    label:hover {
        @apply cursor-pointer;
    }

    
</style>