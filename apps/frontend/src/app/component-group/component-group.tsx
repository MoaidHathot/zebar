import { createEffect, on, onCleanup, onMount } from 'solid-js';

import template from './component-group.njk?raw';
import { ComponentConfig, ComponentGroupConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { ClockComponent } from '~/components/clock/clock-component';
import { insertAndReplace } from '~/shared/utils';

export interface ComponentGroupProps {
  id: string;
  config: ComponentGroupConfig;
}

export function ComponentGroup(props: ComponentGroupProps) {
  const tempId = `group-${Math.random().toString().slice(2)}`;
  const element = document.createElement('div');
  element.id = tempId;

  createEffect(
    on(
      () => props.config,
      () => {
        const dispose = insertAndReplace(document.getElementById(tempId)!, () =>
          parseTemplate(template, getBindings()),
        );
        onCleanup(() => dispose());
      },
    ),
  );

  function getComponentType(id: string, componentConfig: ComponentConfig) {
    switch (componentConfig.type) {
      case 'clock':
        return <ClockComponent id={id} config={componentConfig} />;
      case 'cpu':
        return <p>Not implemented.</p>;
      default:
        throw new Error(
          `Unknown component type '${
            (componentConfig as ComponentConfig).type
          }'.`,
        );
    }
  }

  function getBindings() {
    return {
      strings: {
        root_props: `id="${tempId}" data-root="true"`,
      },
      components: {
        components: () =>
          props.config.components.map(componentConfig =>
            getComponentType('temp', componentConfig),
          ),
      },
    };
  }

  onMount(() => console.log('ComponentGroup mounted'));
  onCleanup(() => console.log('ComponentGroup cleanup'));

  return element;
}
