import {List, Card, Image} from '@douyinfe/semi-ui';
import {invoke} from '@tauri-apps/api'
import {useEffect, useState} from 'react';

export default function Wallpaper() {
    const [data, setData] = useState<any>([])
    const get_page = async (index: number, page_number: number) => {
        const temp: any = await invoke('get_photo', {
            page: {
                index: index,
                page_number: page_number,
                type_of: "BingList"
            }
        })
        console.log(temp)
        setData(temp?.result)
    }
    useEffect(() => {
        get_page(1, 10)
    }, [])
    return (
        <div><List
            grid={{
                gutter: 12,
                xs: 0,
                sm: 0,
                md: 12,
                lg: 8,
                xl: 8,
                xxl: 6,
            }}
            dataSource={data}
            renderItem={item => (
                <List.Item>
                    <Card
                        style={{
                            width: 400,
                            height: 360,
                            margin: '1rem'
                        }}
                        cover={
                            <Image
                                width={400}
                                height={240}
                                preview={{
                                    src: item?.link
                                }}
                                alt={item.title}
                                src={item?.link}
                            />
                        }
                    >
                        <Card.Meta
                            title={item.title}
                            description={item?.content}
                        />

                        {/*<div style={{margin: '12px 0', display: 'flex', justifyContent: 'flex-end'}}>*/}
                        {/*    <ButtonGroup theme="borderless" style={{marginTop: 8}}>*/}
                        {/*        <Button>编辑</Button>*/}
                        {/*        <Button>更多</Button>*/}
                        {/*    </ButtonGroup>*/}
                        {/*</div>*/}
                    </Card>
                </List.Item>
            )}
        />
        </div>
    )
}