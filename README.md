# Occupation Detection AI

While on a crowded restaurant I noticed it was very difficult for the waiters to understand how much time a table went without any attention.

As a good SWE, I decide to try my luck with a tech solution to this problem (that probably would be better solved without tech). This repository is an attempt at that.


# The Algorithm

The core idea is not to build a super intelligent ML model that understand context, but rather to detect moving
bodies and if they are interacting with a table (for example), if a person gets into a position, sits at the table, this table is then occupied. It is trivial how an application would be made using this on a restaurant system, but this repo only busies itself with this problem: Detecting bodies and detect if they are fixed in some region of the camera.

EX:
On a given scene, the model should:
1. Detect all the people in movement in a scene (don't care about people who stay still for the length of the video).
2. Keep track of where these people are once they are detected.

# Preparation of Data
I have choosen the avenue dataset(https://www.cse.cuhk.edu.hk/leojia/projects/detectabnormal/dataset.html) as there are many people moving in it (full body). In order to do supervised learning, the dataset must be annotated and labeled, this has to be done frame by frame, in this section I'll describe the preprocessing steps.


## Preprocessing

The first step is breaking the videos into images, each video will become a folder where each frame is saved as '{VIDEO_INDEX}/{FRAME_INDEX}.png'. This can be done using FFMPEG. EX: ```ffmpeg -i './AvenueDataset/testing_videos/01.avi' -r 25 -f image2 './FrameDataset/01/%07d.png'```

The second step is transforming the N images of each video into N - 1 images of difference (to detect movement), this is the input of the model.
