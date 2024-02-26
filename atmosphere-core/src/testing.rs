//! Provides functions for automated database testing.
//!
//! This module contains asynchronous functions to test the basic CRUD (Create, Read, Update, Delete)
//! operations on database entities. It ensures that these operations are executed correctly and that
//! the data integrity is maintained throughout the process.

use crate::Entity;
use std::fmt::Debug;

/// Tests entity creation in the database.
///
/// Verifies that an entity can be created and retrieved correctly. It asserts the non-existence of
/// the entity before creation and checks for equality between the created and retrieved instances.
pub async fn create<E>(mut instance: E, pool: &crate::Pool)
where
    E: Entity + Clone + Debug + Eq + Send,
{
    assert!(
        E::find(instance.pk(), pool).await.is_err(),
        "instance was found (find) before it was created"
    );

    assert!(
        E::find_optional(instance.pk(), pool)
            .await
            .unwrap()
            .is_none(),
        "instance was found (find_optional) before it was created"
    );

    instance.create(pool).await.expect("insertion did not work");

    let retrieved = E::find(instance.pk(), pool)
        .await
        .expect("instance not found after insertion");

    assert_eq!(instance, retrieved);
}

/// Tests reading of an entity from the database.
///
/// Validates that an entity, once created, can be correctly read from the database. It ensures
/// that the entity does not exist prior to creation and that the retrieved instance matches the
/// created one.
pub async fn read<E>(mut instance: E, pool: &crate::Pool)
where
    E: Entity + Clone + Debug + Eq + Send,
{
    assert!(
        E::find(instance.pk(), pool).await.is_err(),
        "instance was found (find) after deletion"
    );

    assert!(
        E::find_optional(instance.pk(), pool)
            .await
            .unwrap()
            .is_none(),
        "instance was found (find_optional) after deletion"
    );

    assert!(
        E::find_all(pool).await.unwrap().is_empty(),
        "there was an instance found in the database before creating"
    );

    instance.create(pool).await.expect("insertion did not work");

    let retrieved = E::find(instance.pk(), pool)
        .await
        .expect("instance not found after insertion");

    assert_eq!(instance, retrieved);

    assert_eq!(E::find_all(pool).await.unwrap(), vec![instance.clone()]);
}

/// Tests updating of an entity in the database.
///
/// Checks that an entity can be updated and the changes are correctly reflected. Each update is
/// verified by reloading and comparing it with the original instance.
pub async fn update<E>(mut instance: E, updates: Vec<E>, pool: &crate::Pool)
where
    E: Entity + Clone + Debug + Eq + Send,
{
    instance.save(pool).await.expect("insertion did not work");

    for mut update in updates {
        update
            .update(pool)
            .await
            .expect("updating the instance did not work");

        instance
            .reload(pool)
            .await
            .expect("reloading the instance did not work");

        assert_eq!(instance, update);

        let retrieved = E::find(instance.pk(), pool)
            .await
            .expect("instance not found after update");

        assert_eq!(instance, retrieved);

        let retrieved = E::find_optional(instance.pk(), pool)
            .await
            .unwrap()
            .expect("instance not found (find_optional) after update");

        assert_eq!(instance, retrieved);
    }
}

/// Tests deletion of an entity from the database.
///
/// Ensures that an entity can be deleted and is no longer retrievable post-deletion. It also
/// confirms the non-existence of the entity after a delete operation.
pub async fn delete<E>(mut instance: E, pool: &crate::Pool)
where
    E: Entity + Clone + Debug + Eq + Send,
{
    instance.create(pool).await.expect("insertion did not work");

    instance.delete(pool).await.expect("deletion did not work");

    instance
        .reload(pool)
        .await
        .expect_err("instance could be reloaded from db after deletion");

    assert!(
        E::find(instance.pk(), pool).await.is_err(),
        "instance was found (find) after deletion"
    );

    assert!(
        E::find_optional(instance.pk(), pool)
            .await
            .unwrap()
            .is_none(),
        "instance was found (find_optional) after deletion"
    );

    instance.create(pool).await.expect("insertion did not work");

    E::delete_by(instance.pk(), pool)
        .await
        .expect("deletion did not work");

    instance
        .reload(pool)
        .await
        .expect_err("instance could be reloaded from db after deletion");
}

// TODO: provide helpers to autogenerate uuids, pks, strings, emails, etc – maybe reexport another
// crate?
