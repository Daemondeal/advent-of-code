#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node node_t;

#define PART 2

#if PART == 1
    #define MAX 9
#else
    #define MAX 1000000
#endif

#if PART == 1
    #define ITERS 100
#else
    #define ITERS 10000000
#endif

struct node {
    node_t *next;
    int val;
};

node_t *allocate_node(char *labeling, node_t *cache[]) {
    node_t *head = (node_t*)malloc(sizeof(node_t));
    head->val = labeling[0] - '0';
    cache[head->val] = head;
    int len = strlen(labeling);

    node_t *cur = head;
    for (int i = 1; i < len; i++) {
        node_t *new = (node_t*)malloc(sizeof(node_t));
        new->val = labeling[i] - '0';
        cur->next = new;

        cur = new;
        cache[new->val] = new;
    }

    if (PART == 2) {
        for (int i = 10; i <= MAX; i++) {
            node_t *new = (node_t*)malloc(sizeof(node_t));
            new->val = i;
            cur->next = new;

            cur = new;
            cache[i] = new;
        }
    }

    cur->next = head;

    return head;
}

void free_list(node_t *head) {
    int h;
    node_t *p, *next;
        
    h = head->val;
    for (p = head->next; p->val != h; p = next) {
        next = p->next;
        free(p);
    }

    free(head);
}

void print_list(node_t *p, int n) {
    for (int i = 0; i < n; i++) {
        printf("%d ", p->val);
        p = p->next;
    }
    printf("\n");
}

node_t *find_node(node_t *node, int val) {
    while (node->val != val && val > 0) {
        node = node->next;
    }

    return node;
}


int main() {
    node_t *head;
    node_t **cache = (node_t**)malloc(sizeof(node_t*) * (MAX + 1));

    printf("Allocating...\n");
    head = allocate_node("157623984", cache);
    // head = allocate_node("389125467", cache);

    printf("Starting...\n");

    node_t *current = head;
    for (int i = 0; i < ITERS; i++) {

        node_t *taken_head = current->next;
        node_t *taken_tail = taken_head->next->next;

        node_t *over = taken_tail->next;

        current->next = over;

        int dest = current->val - 1;
        if (dest <= 0)
            dest = MAX;
        while (
            taken_head->val == dest || 
            taken_head->next->val == dest || 
            taken_tail->val == dest
            ) {
            dest--;

            if (dest <= 0)
                dest = MAX;
        }

        // node_t *dest_node = find_node(current, dest); // Slow

        node_t *dest_node = cache[dest];
        node_t *dest_next = dest_node->next;
        
        dest_node->next = taken_head;

        taken_tail->next = dest_next;

        current = current->next;
    }

    node_t *final = cache[1]; //find_node(head, 1);

    print_list(final, 9);
    unsigned long long first  = final->next->val;
    unsigned long long second  = final->next->next->val;

    printf("%llu * %llu = %llu\n", first, second, first * second);


    free(cache);
    free_list(head);
}