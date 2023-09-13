
import numpy
import os
import PIL
import pyautogui
import time

CACHE_PATH = os.path.join(os.path.dirname(__file__), "cache")

class ScreenImage:

    def __init__(self, name) -> None:
        self.name = name
        self.origin = None
        self.size = None
        self.img = None
        self.img_nparray = None

    def take_start(self, x, y):
        self.origin = (x, y)
    
    def take_end(self, x, y):
        self.size = (x - self.origin[0], y - self.origin[1])
        self.img = self.grab_image_at_coords()
        self.img_nparray = numpy.array(self.img)

        self.img.save(os.path.join(CACHE_PATH, self.name))

    def grab_image_at_coords(self):
        return pyautogui.screenshot(region=(*self.origin, *self.size))

    def is_on_screen(self):
        return images_match(self.img_nparray, self.grab_image_at_coords())
        
    def wait_until_visible(self):
        while not self.is_on_screen():
            time.sleep(0.25)


def await_screen_update():
    original = numpy.array(pyautogui.screenshot())

    while not was_screen_updated(original):
        time.sleep(0.1)


def was_screen_updated(original):
    if type(original) is PIL.Image:
        original = numpy.array(original)
    assert type(original) is numpy.ndarray, "Original image is not a numpy NDArray"

    current = numpy.array(pyautogui.screenshot())

    return not images_match(current, original)


def images_match(a, b):
    if type(a) is PIL.Image:
        a = numpy.array(a)
    
    if type(b) is PIL.Image:
        b = numpy.array(b)

    return numpy.max(numpy.abs(a - b)) == 0
