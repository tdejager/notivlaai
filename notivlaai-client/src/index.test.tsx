import React from 'react';
import ReactDOM from 'react-dom';
import Enzyme, { mount } from 'enzyme';
import Adapter from 'enzyme-adapter-react-16';
import App from './App';
import setupStore from './store';
import { OrderContainer } from './components';
import { OrderComponent } from './OrderComponent';
import { OrderType, VlaaiType } from './types';

Enzyme.configure({ adapter: new Adapter() });

const testData: OrderType = {
  id: 0,
  customerName: 'Tim de Jager',
  rows: [
    {
      vlaai: VlaaiType.Kers,
      amount: 3,
    },
    {
      vlaai: VlaaiType.Abrikoos,
      amount: 3,
    },
  ],
};

describe('App', () => {
  it('renders', () => {
    expect.assertions(1);
    const [useStoreHook] = setupStore();
    const div = document.createElement('div');
    ReactDOM.render(<App demo={false} useStore={useStoreHook} />, div);
    expect(true).toBe(true);
  });

  it('list should be empty if the store is', () => {
    expect.assertions(2);
    const [useStoreHook] = setupStore();
    const component = mount(<App demo={false} useStore={useStoreHook} />);
    expect(component.find(OrderContainer)).toHaveLength(1);
    expect(component.find(OrderComponent)).toHaveLength(0);
  });

  it('should have orders if the store has orders', () => {
    expect.assertions(1);

    const [useStoreHook, api] = setupStore();
    // Set some test data
    api.setState({ orders: [testData] });
    const component = mount(<App demo={false} useStore={useStoreHook} />);
    expect(component.find(OrderComponent)).toHaveLength(1);
  });

  it('should disappear when it is clicked', () => {
    expect.assertions(1);

    const [useStoreHook, api] = setupStore();
    // Set some test data
    api.setState({ orders: [testData] });

    // Disable animations so we don't have to wait for the completion
    const component = mount(<App demo={false} disableAnimations useStore={useStoreHook} />);
    component.find('OrderComponent Button').simulate('click');
    component.update();
    expect(component.find(OrderComponent)).toHaveLength(0);
  });
});
