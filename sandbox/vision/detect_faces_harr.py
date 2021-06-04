"""Simple webcam face detection using harr cascades."""

import cv2

# Tune following https://stackoverflow.com/a/20805153/14928812
SCALE_FACTOR=1.1
MIN_NEIGHBORS=4

face_classifier = cv2.CascadeClassifier('/usr/share/opencv4/haarcascades/haarcascade_frontalface_default.xml')
eye_classifier = cv2.CascadeClassifier('/usr/share/opencv4/haarcascades/haarcascade_eye.xml')

camera = cv2.VideoCapture("v4l2src device=/dev/video0 ! video/x-raw,format=YUY2,width=640,height=480,framerate=30/1 ! videoconvert ! video/x-raw, format=BGR ! appsink drop=1 ", cv2.CAP_GSTREAMER)

while True:
    _, frame = camera.read()

    # Convert frame to greyscale and detect faces using the harr cascade
    # classifier.
    frame_grayscale = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    faces = face_classifier.detectMultiScale(frame_grayscale, SCALE_FACTOR,
            MIN_NEIGHBORS)
    eyes = eye_classifier.detectMultiScale(frame_grayscale, SCALE_FACTOR,
            MIN_NEIGHBORS)


    # Draw a rectangle around the discovered faces and show the results
    for (x, y, w, h) in faces:
        cv2.rectangle(frame, (x, y), (x+w, y+h), (255, 0, 0,), 2)

    # Draw a rectangle around the discovered eyes and show the results
    for (x, y, w, h) in eyes:
        cv2.rectangle(frame, (x, y), (x+w, y+h), (0, 255, 0,), 2)

    cv2.imshow('frame', frame)
    if cv2.waitKey(1) == ord('q'):
        break

camera.release()
cv2.destroyAllWindows()
