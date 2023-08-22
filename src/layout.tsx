import { Button, Empty, Layout, Nav } from '@douyinfe/semi-ui';
import { IconHome, IconSetting } from '@douyinfe/semi-icons';
import styles from '@/style/layout.module.less';
import { Route, Routes } from 'react-router-dom';
import { IllustrationConstruction, IllustrationConstructionDark } from '@douyinfe/semi-illustrations';
import lazyload from '@/utils/lazyload.tsx';

const WallPaperComponent = lazyload(() => import("@/page/wallpaper"))

const { Header, Content } = Layout;
export default function LayoutPage() {
    return <>
        <Layout className={styles['layout']}>
            <Header style={{ backgroundColor: 'var(--semi-color-bg-1)' }}>
                    <Nav mode="horizontal" defaultSelectedKeys={['Home']}>
                        <Nav.Item itemKey="Home" text="Bing" icon={<IconHome size="large" />} />
                        <Nav.Footer>
                            <Button
                                theme="borderless"
                                icon={<IconSetting size="large" />}
                            />
                        </Nav.Footer>
                    </Nav>
            </Header>
            <Content className={styles['content']}>
                 <Routes>
                     <Route path={'/'} element={<WallPaperComponent />}/>

                     <Route path="*" element={
                         <Empty
                             className={styles['empty']}
                             image={<IllustrationConstruction style={{ width: 300, height: 300 }} />}
                             darkModeImage={<IllustrationConstructionDark style={{ width: 300, height: 300 }} />}
                             title={'功能建设中'}
                             description="当前功能暂未开放，敬请期待。"
                         />
                     } />
                 </Routes>
            </Content>
        </Layout>
    </>

}