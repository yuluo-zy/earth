import loadable from '@loadable/component';
import { Spin } from '@douyinfe/semi-ui';
import React, { ElementType } from 'react';

const spin: React.CSSProperties = {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: '100%',
    minHeight: ' calc(100vh - 200px)',
};

function load(fn: any, options: any): any {
    const Component  = loadable(fn, options);
    Component.preload = fn?.requireAsync || fn;
    return Component ;
}

function LoadingComponent(props: {
    error: boolean;
    timedOut: boolean;
    pastDelay: boolean;
}) {
    if (props.error) {
        return null;
    }
    return (
        <div style={spin}>
            <Spin />
        </div>
    );
}

export default (loader: () => Promise<any>): ElementType =>
    load(loader, {
        fallback: LoadingComponent({
            pastDelay: true,
            error: false,
            timedOut: false,
        }),
    });
