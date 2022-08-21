import { open } from '@tauri-apps/api/dialog';
import { appDir, desktopDir } from '@tauri-apps/api/path';

export async function selectDir(): Promise<string | null> {
    const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: await appDir(),
    });
    if (Array.isArray(selected)) {
        // user selected multiple directories
        return null;
    } else if (selected === null) {
        // user cancelled the selection
        return null;
    } else {
        // user selected a single directory
        return selected;
    };
}

export async function selectFile() {
    const selected = await open({
        directory: false,
        multiple: false,
        defaultPath: await desktopDir(),
    });
    if (Array.isArray(selected)) {
        // user selected multiple directories
        return null;
    } else if (selected === null) {
        // user cancelled the selection
        return null;
    } else {
        // user selected a single directory
        return selected;
    };
}