
from enum import Enum, auto
from typing import Any
import pynput
import pyautogui

class MouseInputState(Enum):

    NeedsTrained = auto()

    GetMainHeader1 = auto()
    GetMainHeader2 = auto()
    GetOrderInput = auto()
    GetDeleteButton = auto()

    GetErrorHeader1 = auto()
    GetErrorHeader2 = auto()

    AwaitingOpSelection = auto()

    FullyTrained = auto()

class MouseHandler:

    def __init__(self) -> None:
        self.click_callbacks = list()

        self.listener = pynput.mouse.Listener(
            on_click=self._handle_click,
        )


    def __enter__(self):
        self.listener.start()
        self.listener.wait()


    def __exit__(self, exc_type, exc_value, traceback):
        self.listener.stop()


    def click(self, x, y):
        ox, oy = pyautogui.position() # original x and y

        pyautogui.click(x=x, y=y)
        pyautogui.moveTo(ox, oy)


    def _handle_click(self, x, y, button, pressed):
        # black list ops that are not left-click
        if not (button == pynput.mouse.Button.left and pressed):
            return

        for callback in self.click_callbacks:
            callback(x, y)
        
        self.click_callbacks = [fn for fn in self.click_callbacks if fn.permanent]

    def add_click_callback(self, fn, permanent=False):
        self.click_callbacks.append(
            ClickCallback(fn, permanent=permanent)
        )


class ClickCallback:

    def __init__(self, func, permanent=False) -> None:
        self.func = func
        self.permanent=permanent

    def __call__(self, *args: Any, **kwds: Any) -> Any:
        self.func()
