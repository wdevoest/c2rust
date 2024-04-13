#include <stdio.h>
#include <time.h>
#include <SDL2/SDL.h>

#define SCREEN_WIDTH 1024
#define SCREEN_HEIGHT 576
#define TILE_SIZE 16
#define BOX_X 128
#define BOX_Y 72
#define BOX_W 768
#define BOX_H 432

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
};

typedef struct Box {
    int x;
    int y;
    int w;
    int h;
} Box;

typedef struct SnakeCell {
    int x;
    int y;
    int direction;
    struct SnakeCell* next;
} SnakeCell;

typedef struct Apple {
    int x;
    int y;
} Apple;

SnakeCell* createSnake();
void increaseSnake(SnakeCell* head);
void renderSnake(SDL_Renderer* renderer, SnakeCell* head);
void updateSnake(SnakeCell* head, int newDirection, Apple* apple);
int moveSnake(SnakeCell* head, int newDirection);
Box* createBox(int x, int y, int w, int h);
void renderBox(SDL_Renderer*, Box* box);
Apple* createApple();
void renderApple(SDL_Renderer* renderer, Apple* apple);
void updateApple(Apple* apple);

int main(int argc, char** argv)
{
    srand(time(NULL));
    SDL_Window* window;
    SDL_Renderer* renderer;
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
    {
        printf("Could not initialize SDL. Error: %s\n", SDL_GetError());
        return 1;
    }

    window = SDL_CreateWindow("Serpent", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, SCREEN_WIDTH, SCREEN_HEIGHT, SDL_WINDOW_SHOWN);
    if (window == NULL)
    {
        printf("Could not create SDL window. Error: %s\n", SDL_GetError());
        return 1;
    }

    renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    if (renderer == NULL)
    {
        printf("Could not create SDL renderer. Error %s\n", SDL_GetError());
    }

    Box* box = createBox(BOX_X, BOX_Y, BOX_W, BOX_H);
    SnakeCell* snake = createSnake();
    Apple* apple = createApple();

    const int FPS = 60;
    const int frameDelay = 1000 / FPS;
    int frameStart, frameTime;

    int running = 1;
    SDL_Event event;
    while (running)
    {
        frameStart = SDL_GetTicks();

        while (SDL_PollEvent(&event))
        {
            if (event.type == SDL_QUIT)
            {
                running = 0;
            }
            if (event.type == SDL_KEYDOWN)
            {
                if (event.key.keysym.sym == SDLK_ESCAPE)
                {
                    running = 0;
                }
                else if (event.key.keysym.sym == SDLK_UP && snake->direction != DOWN)
                {
                    updateSnake(snake, UP, apple);
                }
                else if (event.key.keysym.sym == SDLK_DOWN && snake->direction != UP)
                {
                    updateSnake(snake, DOWN, apple);
                }
                else if (event.key.keysym.sym == SDLK_LEFT && snake->direction != RIGHT)
                {
                    updateSnake(snake, LEFT, apple);
                }
                else if (event.key.keysym.sym == SDLK_RIGHT && snake->direction != LEFT)
                {
                    updateSnake(snake, RIGHT, apple);
                }
            }
        }

        SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
        SDL_RenderClear(renderer);

        renderBox(renderer, box);
        renderApple(renderer, apple);
        renderSnake(renderer, snake);

        SDL_RenderPresent(renderer);

        frameTime = SDL_GetTicks() - frameStart;
        if (frameDelay > frameTime)
        {
            SDL_Delay(frameDelay - frameTime);
        }
    }

    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
    return 0;
}

SnakeCell* createSnake()
{
    SnakeCell* head = (SnakeCell*)malloc(sizeof(SnakeCell));
    head->x = TILE_SIZE * 30;
    head->y = TILE_SIZE * 20;
    head->direction = UP;
    head->next = NULL;

    increaseSnake(head);
    increaseSnake(head);

    return head;
}

void increaseSnake(SnakeCell* head)
{
    SnakeCell* tmp = head;
    while (tmp->next != NULL)
    {
        tmp = tmp->next;
    }
    SnakeCell* newCell = (SnakeCell*)malloc(sizeof(SnakeCell));
    if (tmp->direction == UP)
    {
        newCell->x = tmp->x;
        newCell->y = tmp->y + TILE_SIZE;
    }
    else if (tmp->direction == DOWN)
    {
        newCell->x = tmp->x;
        newCell->y = tmp->y - TILE_SIZE;
    }
    else if (tmp->direction == LEFT)
    {
        newCell->x = tmp->x + TILE_SIZE;
        newCell->y = tmp->y;
    }
    else if (tmp->direction == RIGHT)
    {
        newCell->x = tmp->x - TILE_SIZE;
        newCell->y = tmp->y;
    }
    newCell->next = NULL;
    tmp->next = newCell;
}

void renderSnake(SDL_Renderer* renderer, SnakeCell* head)
{
    SDL_SetRenderDrawColor(renderer, 55, 175, 175, 255);
    SDL_Rect rect = {head->x, head->y, TILE_SIZE, TILE_SIZE};
    SDL_RenderFillRect(renderer, &rect);
    SnakeCell* tmp = head->next;
    while (tmp != NULL)
    {
        SDL_SetRenderDrawColor(renderer, 0, 200, 0, 255);
        SDL_Rect rect = {tmp->x, tmp->y, TILE_SIZE, TILE_SIZE};
        SDL_RenderFillRect(renderer, &rect);
        tmp = tmp->next;
    }
}

void updateSnake(SnakeCell* head, const int newDirection, Apple* apple)
{
    SnakeCell* tmp = head;
    int prevX = head->x;
    int prevY = head->y;
    int prevD = head->direction;

    if (moveSnake(head, newDirection) == 0)
    {
        head->direction = newDirection;
        int tmpX, tmpY, tmpD;
        while (tmp->next != NULL)
        {
            tmpX = tmp->next->x;
            tmpY = tmp->next->y;
            tmpD = tmp->next->direction;
            tmp->next->x = prevX;
            tmp->next->y = prevY;
            tmp->next->direction = prevD;
            prevX = tmpX;
            prevY = tmpY;
            prevD = tmpD;
            tmp = tmp->next;
        }
    }

    if (head->x == apple->x && head->y == apple->y)
    {
        updateApple(apple);
        increaseSnake(head);
    }
}

int moveSnake(SnakeCell* head, int newDirection)
{
    int newX, newY;
    if (newDirection == UP && head->direction != DOWN)
    {
        newX = head->x;
        newY = head->y - TILE_SIZE;
    }
    else if (newDirection == DOWN && head->direction != UP)
    {
        newX = head->x;
        newY = head->y + TILE_SIZE;
    }
    else if (newDirection == LEFT && head->direction != RIGHT)
    {
        newX = head->x - TILE_SIZE;
        newY = head->y;
    }
    else if (newDirection == RIGHT && head->direction != LEFT)
    {
        newX = head->x + TILE_SIZE;
        newY = head->y;
    }
    else
    {
        return 1;
    }

    int canMove = 1;
    SnakeCell* tmp = head->next;
    while (tmp != NULL)
    {
        if (tmp->x == newX && tmp->y == newY)
        {
            canMove = 0;
        }
        tmp = tmp->next;
    }
    if (canMove)
    {
        if (newDirection == UP)
        {
            head->y -= TILE_SIZE;
            return 0;
        }
        else if (newDirection == DOWN)
        {
            head->y += TILE_SIZE;
            return 0;
        }
        else if (newDirection == LEFT)
        {
            head->x -= TILE_SIZE;
            return 0;
        }
        else if (newDirection == RIGHT)
        {
            head->x += TILE_SIZE;
            return 0;
        }
    }
    return 1;
}

Box* createBox(int x, int y, int w, int h)
{
    Box* box = (Box*)malloc(sizeof(Box));
    box->x = x;
    box->y = y;
    box->w = w;
    box->h = h;
    return box;
}

void renderBox(SDL_Renderer* renderer, Box* box)
{
    SDL_SetRenderDrawColor(renderer, 50, 50, 50, 255);
    SDL_Rect rect = {box->x, box->y, box->w, box->h};
    SDL_RenderDrawRect(renderer, &rect);
}

Apple* createApple()
{
    Apple* apple = (Apple*)malloc(sizeof(Apple));
    int remainder;
    int x = (rand() % (BOX_W - BOX_X + 1)) + BOX_X;
    remainder = x % TILE_SIZE;
    if (remainder < TILE_SIZE / 2)
    {
        x -= remainder;
    }
    else
    {
        x += TILE_SIZE - remainder;
    }
    int y = (rand() % (BOX_H - BOX_Y + 1)) + BOX_Y;
    remainder = y % TILE_SIZE;
    if (remainder < TILE_SIZE / 2)
    {
        y -= remainder;
    }
    else
    {
        y += TILE_SIZE - remainder;
    }
    apple->x = x;
    apple->y = y;
    return apple;
}

void renderApple(SDL_Renderer* renderer, Apple* apple)
{
    SDL_SetRenderDrawColor(renderer, 200, 0, 0, 255);
    SDL_Rect rect = {apple->x, apple->y, TILE_SIZE, TILE_SIZE};
    SDL_RenderFillRect(renderer, &rect);
}

void updateApple(Apple* apple)
{
    int remainder;
    int x = (rand() % (BOX_W - BOX_X + 1)) + BOX_X;
    remainder = x % TILE_SIZE;
    if (remainder < TILE_SIZE / 2)
    {
        x -= remainder;
    }
    else
    {
        x += TILE_SIZE - remainder;
    }
    int y = (rand() % (BOX_H - BOX_Y + 1)) + BOX_Y;
    remainder = y % TILE_SIZE;
    if (remainder < TILE_SIZE / 2)
    {
        y -= remainder;
    }
    else
    {
        y += TILE_SIZE - remainder;
    }
    apple->x = x;
    apple->y = y;
}