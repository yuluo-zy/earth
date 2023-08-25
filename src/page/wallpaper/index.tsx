import { invoke } from '@tauri-apps/api'
import { useEffect } from 'react';
export default function Wallpaper() {
        const get_page = async () => {
                console.info("lkjlkj")
                return await invoke('get_photo', {page:{
                        index: 1,
                        page_number: 1,
                        type_of: "BingList"
                }})
        }
        useEffect( ()=> {
                get_page().then(r => {
                        console.info(r)
                })
        }, [])
        return (
        <div>kjhjh</div>
        )
}